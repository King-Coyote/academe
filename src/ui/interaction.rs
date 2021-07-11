use crate::{game::*, input::*, utils::data_struct::*};
use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
};

use super::ClickHandlers;

#[derive(Default)]
pub struct InteractableOrder {
    pub map: MultiTreeMap<i32, Entity>,
    pub ui_blocking: Option<Entity>,
    pub current: Option<(Entity, f32)>,
}

#[derive(PartialEq, Clone, Copy, Reflect)]
pub enum InteractState {
    Enabled,
    InsideBounds,
    Disabled,
}

#[derive(Reflect)]
#[reflect(Component)]
pub struct InteractableObject {
    pub min_dist: f32, // only really look at things inside this distance. For efficiency
    pub state: InteractState,
    #[reflect(ignore)]
    pub mouse_inside: Option<Box<dyn Fn(&Vec2, &MouseState) -> bool + Send + Sync>>,
}

pub struct InteractableHover {
    pub entity: Entity,
    pub zindex: i32,
}

pub enum ObjectInteraction {
    Enabled,
    Hovered,
    Clicked,
    Disabled,
}

pub struct MouseInside;

impl Default for InteractableObject {
    fn default() -> Self {
        InteractableObject {
            min_dist: 0.0,
            state: InteractState::Enabled,
            mouse_inside: None,
        }
    }
}

// #[derive(Default, PartialEq, Reflect)]
// #[reflect(Component)]
// pub struct ZIndex {
//     pub last: i32,
//     pub current: i32,
// }

// impl ZIndex {
//     pub fn from_transform(t: &Transform) -> Self {
//         ZIndex {
//             last: t.translation.z as i32,
//             current: t.translation.z as i32,
//         }
//     }

//     pub fn update(&mut self, t: &Transform) {
//         self.last = self.current;
//         self.current = t.translation.z as i32;
//     }
// }

// pub fn interactable_zindex(
//     mut commands: Commands,
//     mut order: ResMut<InteractableOrder>,
//     q_interact_link: Query<(Entity, &Transform), (With<InteractableObject>, Without<ZIndex>)>,
//     mut q_interact_change: Query<
//         (&Transform, &mut ZIndex),
//         (With<InteractableObject>, Changed<Transform>),
//     >,
// ) {
//     for (entity, t) in q_interact_link.iter() {
//         let z_index = ZIndex::from_transform(t);
//         multimap_insert(&mut order.map, z_index.current, entity);
//         commands.entity(entity).insert(z_index);
//     }
//     for (t, mut z) in q_interact_change
//         .iter_mut()
//         .filter(|(t, z)| z.current != t.translation.z as i32)
//     {
//         z.update(t);
//     }
// }

// pub fn interactable_zindex_change(
//     mut order: ResMut<InteractableOrder>,
//     q_interact: Query<(Entity, &ZIndex), (With<InteractableObject>, Changed<ZIndex>)>,
// ) {
//     for (entity, z_index) in q_interact.iter() {
//         multimap_remove(&mut order.map, z_index.last, entity);
//         multimap_insert(&mut order.map, z_index.current, entity);
//     }
// }

// pub fn interactable_mouse_inside(
//     mut order: ResMut<InteractableOrder>,
//     mouse: Res<MouseState>,
//     mut er_mousemove: EventReader<CursorMoved>,
//     mut q_interactable: Query<(Entity, &mut InteractableObject, &Transform)>,
// ) {
//     use InteractState::*;
//     for e in er_mousemove.iter() {
//         if order.ui_blocking.is_some() {
//             order.current = None;
//             return;
//         }
//         for (zindex, ent_vec) in order.map.iter().rev() {
//             for ord_entity in ent_vec {
//                 if let Ok((entity, interactable, transform)) =
//                     q_interactable.get_mut(*ord_entity)
//                 {
//                     let pos = Vec2::new(transform.translation.x, transform.translation.y);
//                     let maybe_inside = pos.distance(mouse.world_pos) <= interactable.min_dist;
//                     let inside_fn = interactable.mouse_inside.as_ref().unwrap();
//                     if order.ui_blocking.is_none() && maybe_inside && (inside_fn)(&pos, &*mouse) {
//                         if let Enabled = interactable.state {
//                             order.current = Some(entity);
//                             return;
//                         };
//                     }
//                 }
//             }
//         }
//         order.current = None;
//     }
// }

pub fn object_interaction_ordering(
    mut order: ResMut<InteractableOrder>,
    q_interaction: Query<(Entity, &Transform, &ObjectInteraction), Changed<ObjectInteraction>>,
) {
    use ObjectInteraction::*;
    let mut max_z_index = f32::NEG_INFINITY;
    if order.ui_blocking.is_some() {
        if let Some(current) = order.current {
            info!("Removed {:?} from current order due to blocking ui.", current.0);
            order.current = None;
        }
        return;
    }
    q_interaction
        .iter()
        .fold(None, |acc, (entity, transform, interaction)| {
            match *interaction {
                Hovered => {
                    let z_index = transform.translation.z;
                    if z_index > max_z_index {
                        max_z_index = z_index;
                        return Some(entity);
                    }
                },
                Enabled => {
                    if let Some(current) = order.current {
                        if entity == current.0 {
                            info!("Removed {:?} from current order.", current.0);
                            order.current = None;
                        }
                    }
                },
                _ => {}
            }
            None
        })
        .map(|max_entity| {
            match order.current {
                Some(current) => {
                    if max_z_index > current.1 {
                        info!("{:?} replaced {:?} as current in order.", max_entity, current.0);
                        order.current = Some((max_entity, max_z_index));
                    }
                },
                None => {
                    info!("New current in order: {:?}.", max_entity);
                    order.current = Some((max_entity, max_z_index));
                }
            }
            Some(())
        });
}

pub fn deleteme_ui_test(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(300.0), Val::Px(300.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::ZERO),
        material: materials.add(Color::GRAY.into()),
        ..Default::default()
    })
    ;
}

pub fn interactable_handling(
    mut commands: Commands,
    mouse: Res<MouseState>,
    order: Res<InteractableOrder>,
    mut er_mouseinput: EventReader<MouseButtonInput>,
    q_interact: Query<(&InteractableObject, &ClickHandlers)>,
) {
    // let mut peekable = er_mouseinput.iter().peekable();
    // if order.current.is_none() || peekable.peek().is_none() {
    //     return;
    // }
    // let maybe_query = order
    //     .current
    //     .as_ref()
    //     .and_then(|current| q_interact.get(*current).ok());
    // // this interactable doesn't have any click handlers for some reason
    // if maybe_query.is_none() {
    //     return;
    // }
    // let (interactable, handlers) = maybe_query.unwrap();
    // for e in peekable.into_iter() {
    //     if let ElementState::Pressed = e.state {
    //         continue;
    //     }
    //     match e.button {
    //         MouseButton::Left => {
    //             if let Some(handler) = handlers.left.as_ref() {
    //                 (handler)(&mut commands, &*mouse);
    //             }
    //         }
    //         MouseButton::Right => {
    //             if let Some(handler) = handlers.right.as_ref() {
    //                 (handler)(&mut commands, &*mouse);
    //             }
    //         }
    //         MouseButton::Middle => {}
    //         _ => {}
    //     };
    // }
}

pub fn make_appearance_interactive(
    mut commands: Commands,
    q_appearance: Query<(Entity, &Appearance, &Transform, &Sprite), Without<InteractableObject>>,
) {
    for (entity, _, _, sprite) in q_appearance.iter() {
        let size = sprite.size;
        // wait until it has a size...
        if size.x < f32::EPSILON && size.y < f32::EPSILON {
            continue;
        }
        let max_dim: f32;
        if size.y > size.x {
            max_dim = size.y / 2.0;
        } else {
            max_dim = size.x / 2.0;
        }
        // commands.entity(entity).insert(InteractableObject {
        //     min_dist: max_dim,
        //     mouse_inside: Some(Box::new(move |pos: &Vec2, mouse: &MouseState| -> bool {
        //         let diff = *pos - mouse.world_pos;
        //         diff.x <= size.x / 2.0 && diff.y <= size.y / 2.0
        //     })),
        //     ..Default::default()
        // });
    }
}
