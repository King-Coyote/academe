use std::{
    ops::{Deref, DerefMut,},
};
use bevy::{
    prelude::*,
    reflect::{
        TypeRegistryArc,
    },
    
};

#[derive(Clone, Debug)]
pub enum Target {
    World(Option<Vec2>),
    Screen(Option<Vec2>),
    Entity(Option<Entity>),
}

#[derive(Clone, Debug)]
pub enum GameCommandType {
    Create(String),
    Destroy(String),
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

pub fn execute_game_commands(
    world: &mut World,
) {
    let mut command_queue_res = world.get_resource_mut::<GameCommandQueue>().unwrap();
    if command_queue_res.len() == 0 {
        return;
    }
    let command_queue = command_queue_res.clone();
    command_queue_res.clear();

    let registry = world.get_resource::<TypeRegistryArc>().unwrap().clone();
    let read_registry = registry.read();

    for cmd in command_queue.iter() {
        println!("Executing command: {:?}", cmd);
        let target = match cmd.target {
            Target::World(coords) => {
                let coords = coords.unwrap();
                let pos = Vec3::new(coords.x, coords.y, 0.0);
                world.spawn()
                    .insert(Transform::from_translation(pos))
                    .id()
            },
            Target::Entity(entity) => entity.unwrap(),
            Target::Screen(coords) => {
                let coords = coords.unwrap();
                let pos = Vec3::new(coords.x, coords.y, 0.0);
                world.spawn()
                    .insert(Transform::from_translation(pos))
                    .id()
            }
        };
        use GameCommandType::*;
        match cmd.command {
            Create(ref name) => {
                let rc = read_registry
                    .get_with_short_name(name)
                    .and_then(|reg| {
                        reg.data::<ReflectComponent>()
                    }).unwrap();
                rc.create_component(world, target);
            },
            Destroy(ref name) => {
                let rc = read_registry
                    .get_with_short_name(name)
                    .and_then(|reg| {
                        reg.data::<ReflectComponent>()
                    }).unwrap();
                rc.remove_component(world, target);
            },
        };
    }
}