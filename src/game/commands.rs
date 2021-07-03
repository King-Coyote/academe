use crate::game::*;
use bevy::{
    prelude::*,
    reflect::{DynamicStruct, TypeRegistryArc},
};
use std::{
    fmt,
    ops::{Deref, DerefMut},
    sync::Arc,
};

#[derive(Clone, Debug)]
pub enum Target {
    World(Option<Vec2>),
    Screen(Option<Vec2>),
    Entity(Option<Entity>),
    LastCreated,
}

#[derive(Clone)]
pub enum GameCommandType {
    Create(String),
    Destroy(String),
    Modify {
        name: String,
        values: Arc<DynamicStruct>,
    },
}

impl fmt::Debug for GameCommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GameCommandType::")?;
        use GameCommandType::*;
        match *self {
            Create(ref name) => write!(f, "Create({})", name),
            Destroy(ref name) => write!(f, "Destroy({})", name),
            Modify {
                ref name,
                values: _,
            } => write!(f, "Modify({}, _)", name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameCommand {
    pub target: Target,
    pub command: GameCommandType,
    pub level: u32,
}

#[derive(Clone)]
pub struct GameCommandQueue(pub Vec<GameCommand>);

impl Deref for GameCommandQueue {
    type Target = Vec<GameCommand>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GameCommandQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn execute_game_commands(world: &mut World) {
    let mut command_queue_res = world.get_resource_mut::<GameCommandQueue>().unwrap();
    if command_queue_res.len() == 0 {
        return;
    }
    let command_queue = command_queue_res.clone();
    command_queue_res.clear();

    let registry = world.get_resource::<TypeRegistryArc>().unwrap().clone();
    let read_registry = registry.read();

    let get_rc_by_name = |name: &str| -> &ReflectComponent {
        read_registry
            .get_with_short_name(name)
            .and_then(|reg| reg.data::<ReflectComponent>())
            .unwrap_or_else(|| {
                panic!(
                    "Couldn't get ReflectComponent for name {}; have you forgotten to register it?",
                    name
                )
            })
    };

    let mut last_created: Option<Entity> = None;
    for cmd in command_queue.iter() {
        println!("Executing command: {:?}", cmd);
        let target = match cmd.target {
            Target::World(coords) => {
                let coords = coords.unwrap();
                let pos = Vec3::new(coords.x, coords.y, 0.0);
                let new = world.spawn().insert(Transform::from_translation(pos)).id();
                last_created = Some(new);
                new
            }
            Target::Entity(entity) => entity.unwrap(),
            Target::Screen(coords) => {
                let coords = coords.unwrap();
                let pos = Vec3::new(coords.x, coords.y, 0.0);
                let new = world.spawn().insert(Transform::from_translation(pos)).id();
                last_created = Some(new);
                new
            }
            Target::LastCreated => {
                last_created.expect("Tried to use LastCreated, but there is no entity!")
            }
        };
        use GameCommandType::*;
        match cmd.command {
            Create(ref name) => {
                // get_rc_by_name(name).create_component(world, target);
            }
            Destroy(ref name) => {
                // get_rc_by_name(name).remove_component(world, target);
            }
            Modify {
                ref name,
                ref values,
            } => {
                let mut reflect = get_rc_by_name(name)
                    .reflect_component_mut(world, target)
                    .expect("Could not get reflected component");
                reflect.apply(&**values);
            }
        };
    }
}

// UTILITY FNS
