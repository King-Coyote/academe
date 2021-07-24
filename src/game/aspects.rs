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

#[derive(Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Body {
    pub strength: u32,
    pub endurance: u32,
    pub coordination: u32,
    // something to represent form
}

#[derive(Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Mind {
    pub analysis: u32,
    pub memory: u32,
    pub wit: u32,
}

#[derive(Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Spirit {
    pub charisma: u32,
    pub will: u32,
    pub insight: u32,
}

#[derive(Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Appearance {
    pub entity: Option<Entity>, // whomst does this look like; used for senses in ai
    pub filename: String,
}

pub struct Movement {
    pub level: u32,
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
            .insert(ObjectInteraction::Outside)
            ;
    }
}

pub fn appearance_interact_system(
    mut order: ResMut<InteractableOrder>,
    mouse: Res<MouseState>,
    mut er_cursor: EventReader<CursorMoved>,
    mut q_appearance: Query<(Entity, &Sprite, &Transform, &mut ObjectInteraction), With<Appearance>>,
) {
    use ObjectInteraction::*;
    for e in er_cursor.iter() {
        for (entity, sprite, transform, mut interaction) in q_appearance.iter_mut() {
            let pos = Vec2::new(transform.translation.x, transform.translation.y);
            let diff = pos - mouse.world_pos;
            if order.ui_blocking.is_none()
                && f32::abs(diff.x) <= sprite.size.x * 0.5
                && f32::abs(diff.y) <= sprite.size.y * 0.5
            {
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
