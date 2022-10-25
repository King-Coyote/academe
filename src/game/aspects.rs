use bevy::{
    ecs::{component::Component,},
    prelude::*,
};
use std::marker::PhantomData;
use crate::{
    ui::*,
    input::MouseState,
};

// #[derive(Bundle)]
// pub struct GameComponent<T: Component> {
//     pub level: Level<T>,
//     pub component: T,
// }

// impl<T: Component> GameComponent<T> {
//     fn new(level: u32, component: T) -> Self {
//         let game_component: GameComponent<T> = GameComponent {
//             level: Level(level, PhantomData),
//             component,
//         };
//         game_component
//     }
// }

pub struct Level<T: Component>(u32, PhantomData<T>);

#[derive(Component)]
pub struct Body {
    pub strength: u32,
    pub endurance: u32,
    pub coordination: u32,
    // something to represent form
}

#[derive(Component)]
pub struct Mind {
    pub analysis: u32,
    pub memory: u32,
    pub wit: u32,
}

#[derive(Component)]
pub struct Spirit {
    pub charisma: u32,
    pub will: u32,
    pub insight: u32,
}

#[derive(Default, Component)]
pub struct Appearance {
    pub entity: Option<Entity>, // whomst does this look like; used for senses in ai
    pub filename: String,
}

#[derive(Component)]
pub struct Movement {
    pub level: u32,
}

// TODO is this really needed? Maybe you should just make a bundle for it instead of having the overhead of a whole ass new system. unless this is specifically for magic stuff?...
// this also adds the ability to interact with creatures etc
pub fn appearance_added(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    mut q_appearance: Query<(Entity, &mut Appearance, &Transform), Added<Appearance>>,
) {
    for (entity, mut appearance, transform) in q_appearance.iter_mut() {
        let sprite_bundle = SpriteBundle {
            texture: asset_server.load(appearance.filename.as_str()),
            transform: *transform,
            ..Default::default()
        };
        appearance.entity = Some(entity);
        commands.entity(entity)
            .insert_bundle(sprite_bundle)
            .insert(ObjectInteraction::Outside)
            ;
    }
}

// TODO move this to the interaction module
pub fn appearance_interact_system(
    order: ResMut<InteractableOrder>,
    mouse: Res<MouseState>,
    images: Res<Assets<Image>>,
    mut er_cursor: EventReader<CursorMoved>,
    mut q_appearance: Query<(Entity, &Handle<Image>, &Transform, &mut ObjectInteraction), With<Appearance>>,
) {
    use ObjectInteraction::*;
    for e in er_cursor.iter() {
        for (entity, handle, transform, mut interaction) in q_appearance.iter_mut() {
            let pos = Vec2::new(transform.translation.x, transform.translation.y);
            let diff = pos - mouse.world_pos;
            let image = images.get(handle);
            let is_inside = || -> Option<()> {
                let inside = order.ui_blocking.is_none()
                    && f32::abs(diff.x) <= (image?.texture_descriptor.size.width as f32) * 0.5
                    && f32::abs(diff.y) <= (image?.texture_descriptor.size.height as f32) * 0.5;
                if !inside {
                    return None;
                }
                Some(())
            };
            if is_inside().is_some() {
                *interaction = Inside;
            } else {
                *interaction = Outside;
            }
        }
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
