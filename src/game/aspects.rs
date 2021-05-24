use std::{
    marker::PhantomData,
};
use bevy::{
    prelude::*,
    ecs::{
        component::Component,
        reflect::ReflectComponent,
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

#[derive(Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Mind {
    pub analysis: u32,
    pub memory: u32,
    pub wit: u32,
}

#[derive(Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Spirit {
    pub charisma: u32,
    pub will: u32,
    pub insight: u32,
}

#[derive(Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Appearance; // wat do