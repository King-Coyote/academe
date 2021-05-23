use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut,},
};
use bevy::{
    prelude::*,
    ecs::{
        component::Component,
        reflect::ReflectComponent,
        system::{CommandQueue,},
    },
    reflect::{
        TypeRegistryArc,
        serde::{ReflectSerializer, ReflectDeserializer,}
    },
    
};

#[derive(Bundle)]
pub struct GameComponent<T: Component> {
    pub level: Level<T>,
    pub component: T,
}

impl<T: Component> GameComponent<T> {
    fn new(level: u32, component: T) -> Self {
        let game_component: GameComponent<T> = GameComponent {
            level: Level(level, PhantomData),
            component
        };
        game_component
    }
}


pub struct Level<T: Component>(u32, PhantomData<T>);

#[derive(Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Body {
    pub strength: u32,
    pub endurance: u32,
    pub coordination: u32,
    // something to represent form
}

#[derive(Reflect)]
pub struct Mind {
    pub analysis: u32,
    pub memory: u32,
    pub wit: u32,
}

#[derive(Reflect)]
pub struct Spirit {
    pub charisma: u32,
    pub will: u32,
    pub insight: u32,
}

#[derive(Reflect)]
pub struct Appearance; // wat do

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

pub struct GamePlugin;

fn startup_test(
    world: &mut World,
) {
    let body = Body {
        strength: 10,
        endurance: 10,
        coordination: 10,
    };
    let registry = world.get_resource_mut::<TypeRegistryArc>().unwrap().clone();
    let read_registry = registry.read();

    let serializer = ReflectSerializer::new(&body, &read_registry);
    let serialized = ron::ser::to_string_pretty(&serializer, ron::ser::PrettyConfig::default()).unwrap();
    println!("{}", serialized);

    let deser_reflect = ReflectDeserializer::new(&read_registry);
    let mut deser = ron::de::Deserializer::from_str(&serialized).unwrap();
    let deserialized = bevy::reflect::erased_serde::private::serde::de::DeserializeSeed::deserialize(deser_reflect, &mut deser).unwrap();
    let name = deserialized.type_name();
    let entity = world.spawn().id();
    let reflect_comp = read_registry
        .get_with_name(name)
        .and_then(|registration| {
            registration.data::<ReflectComponent>()
        }).unwrap();
    // let durr = reflect_comp.add_component(world, entity, &*deserialized);
    // reflect_comp.remove_component(world, entity);

    let rc = read_registry
        .get_with_short_name("Body")
        .and_then(|reg| {
            reg.data::<ReflectComponent>()
        }).unwrap();
    rc.create_component(world, entity);
    rc.remove_component(world, entity);
}

fn body_test(
    query: Query<&Body>
) {
    for b in query.iter() {
        println!("u have one lmao: {:?}", b);
    }
}

fn execute_game_commands(
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

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .register_type::<Body>()
            .insert_resource(GameCommandQueue(vec![]))
            // .add_startup_system(startup_test.exclusive_system())
            .add_system(execute_game_commands.exclusive_system())
            .add_system(body_test.system())
        ;
    }
}