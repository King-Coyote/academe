use std::{
    marker::PhantomData,
};
use bevy::{
    ecs::{
        component::Component,
        reflect::ReflectComponent,
    }, 
    prelude::*,
};
use crate::{
    ui::{InteractableObject},
    input::MouseState,
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
pub struct Appearance{
    pub entity: Option<Entity>, // whomst does this look like; used for senses in ai
    pub filename: String,
}

// this also adds the ability to interact with creatures etc
pub fn appearance_added(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut q_appearance: Query<(Entity, &mut Appearance, &Transform), Added<Appearance>>,
) {
    for (entity, mut appearance, transform) in q_appearance.iter_mut() {
        let handle = assets.load::<Texture, _>(appearance.filename.as_str());
        let sprite_bundle = SpriteBundle {
            material: materials.add(handle.into()),
            transform: *transform,
            ..Default::default()
        };
        appearance.entity = Some(entity);
        commands.entity(entity)
            .insert_bundle(sprite_bundle)
            ;
    }
}

// pub fn appearance_changed(
//     assets: Res<AssetServer>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut q_appearance: Query<(&mut Handle<ColorMaterial>, &Appearance), Changed<Appearance>>
// ) {
//     for (mut material, appearance) in q_appearance.iter_mut() {
//         let new_handle = assets.load(appearance.filename.as_str());
//         *material = new_handle.into();
//     }
// }