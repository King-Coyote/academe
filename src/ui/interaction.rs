use crate::{
    game::*,
    input::*,
    utils::entities::do_for_children,
};
use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
};

use super::ClickHandlers;

#[derive(Default)]
pub struct InteractableOrder {
    pub ui_blocking: Option<Entity>,
    pub current: Option<(Entity, f32)>,
}

#[derive(PartialEq)]
pub enum ObjectInteraction {
    Outside,
    Inside,
    Disabled,
}

impl Default for ObjectInteraction {
    fn default() -> Self {
        ObjectInteraction::Outside
    }
}

pub struct ObjectHovered {
    pub entity: Entity,
    pub z_index: f32,
}

pub fn object_interaction_ordering(
    mut order: ResMut<InteractableOrder>,
    q_interaction: Query<(Entity, &Transform, &ObjectInteraction, Option<&Children>)>,
    mut q_interaction_children: Query<(&Parent, &mut Visible)>
) {
    let mut max_z_index = f32::NEG_INFINITY;
    let old = order.current.map(|current| current.0);
    let mut current_children: Option<&Children> = None;
    order.current = q_interaction
        .iter()
        .filter(|(_, _, interaction, children)| **interaction == ObjectInteraction::Inside)
        .fold(None, |acc, (entity, transform, interaction, children)| {
            if transform.translation.z > max_z_index {
                max_z_index = transform.translation.z;
                current_children = children;
                return Some((entity, max_z_index));
            }
            acc
        });
    match (old, order.current) {
        (Some(old), Some(current)) => {
            if old != current.0 {
                info!("Replaced {:?} with {:?} in interactables order.", old, current.0);
                let (_, _, _, current_children) = q_interaction.get(current.0).unwrap();
                let (_, _, _, old_children) = q_interaction.get(old).unwrap();
                set_child_visibility(current_children, &mut q_interaction_children, true);
                set_child_visibility(old_children, &mut q_interaction_children, false);       
            }
        },
        (None, Some(current)) => {
            info!("New current in order: {:?}", current.0);
            let (_, _, _, children) = q_interaction.get(current.0).unwrap();
            set_child_visibility(children, &mut q_interaction_children, true);
        },
        (Some(old), None) => {
            info!("Removed {:?} from order; order current is now blank.", old);
            let (_, _, _, children) = q_interaction.get(old).unwrap();
            set_child_visibility(children, &mut q_interaction_children, false);
        },
        _ => {}
    }
}

fn set_child_visibility(children: Option<&Children>, query: &mut Query<(&Parent, &mut Visible)>, is_visible: bool) {
    if let Some(children) = children {
        for entity in children.iter() {
            if let Ok((_, mut visible)) = query.get_mut(*entity) {
                info!("removing visibility for: {:?}", entity);
                visible.is_visible = is_visible;
                visible.is_transparent = !is_visible;
            }
        }
    }
}

pub fn object_interaction_handling(
    mut commands: Commands,
    mouse: Res<MouseState>,
    order: Res<InteractableOrder>,
    mut er_mouseinput: EventReader<MouseButtonInput>,
    q_interact: Query<(&ObjectInteraction, &ClickHandlers)>,
) {
    if order.current.is_none() {
        return;
    }
    let maybe_query = order
        .current
        .as_ref()
        .and_then(|current| q_interact.get(current.0).ok());
    // this interactable doesn't have any click handlers for some reason
    if maybe_query.is_none() {
        return;
    }
    let (interactable, handlers) = maybe_query.unwrap();
    for e in er_mouseinput.iter() {
        if let ElementState::Pressed = e.state {
            continue;
        }
        match e.button {
            MouseButton::Left => {
                if let Some(handler) = handlers.left.as_ref() {
                    (handler)(&mut commands, &*mouse);
                }
            }
            MouseButton::Right => {
                if let Some(handler) = handlers.right.as_ref() {
                    (handler)(&mut commands, &*mouse);
                }
            }
            MouseButton::Middle => {}
            _ => {}
        };
    }
}

pub fn make_appearance_interactive(
    commands: Commands,
    q_appearance: Query<(Entity, &Appearance, &Transform, &Sprite)>,
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
        // commands.entity(entity).insert(ObjectInteraction {
        //     min_dist: max_dim,
        //     mouse_inside: Some(Box::new(move |pos: &Vec2, mouse: &MouseState| -> bool {
        //         let diff = *pos - mouse.world_pos;
        //         diff.x <= size.x / 2.0 && diff.y <= size.y / 2.0
        //     })),
        //     ..Default::default()
        // });
    }
}
