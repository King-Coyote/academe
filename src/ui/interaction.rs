use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut,},
};
use bevy::{
    prelude::*,
    ecs::component::Component,
    input::{
        ElementState,
        mouse::{MouseButtonInput},
    },
};
use crate::{
    input::*,
    game::*,
};

use super::ClickHandlers;

#[derive(Default)]
pub struct InteractableOrder{
    pub map: BTreeMap<i32, Entity>,
    pub ui_blocking: Option<Entity>,
    pub current: Option<Entity>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum InteractState {
    Enabled,
    InsideBounds,
    Disabled,
}

pub struct InteractableObject{
    pub min_dist: f32, // only really look at things inside this distance. For efficiency
    pub state: InteractState,
    pub mouse_inside: Option<Box<dyn Fn(&Vec2, &MouseState) -> bool + Send + Sync>>,
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

#[derive(PartialEq)]
pub struct ZIndex(i32);
impl_deref_mut!(ZIndex, i32);

pub fn interactable_zindex(
    mut commands: Commands,
    mut order: ResMut<InteractableOrder>,
    q_interact_link: Query<(Entity, &Transform), (With<InteractableObject>, Without<ZIndex>)>,
    mut q_interact_change: Query<(&Transform, &mut ZIndex), (With<InteractableObject>, Changed<Transform>)>,
) {
    for (entity, t) in q_interact_link.iter() {
        let z_index = t.translation.z as i32;
        commands.entity(entity).insert(ZIndex(z_index));
        order.map.insert(z_index, entity);
    }
    for (t, mut z) in q_interact_change.iter_mut().filter(|q| q.1.0 != q.0.translation.z as i32) {
        z.0 = t.translation.z as i32;
    }
}

pub fn interactable_zindex_change(
    mut order: ResMut<InteractableOrder>,
    q_interact: Query<(Entity, &ZIndex), (With<InteractableObject>, Changed<ZIndex>)>,
) {
    for (entity, z_index) in q_interact.iter() {
        order.map.retain(|_, v| entity != *v);
        order.map.insert(z_index.0, entity);
    }
}

pub fn interactable_mouse_inside(
    mut order: ResMut<InteractableOrder>,
    mouse: Res<MouseState>,
    mut er_mousemove: EventReader<CursorMoved>,
    mut q_interactable: Query<(Entity, &mut InteractableObject, &Transform)>,
) {
    use InteractState::*;
    for e in er_mousemove.iter() {
        if order.ui_blocking.is_some() {
            order.current = None;
            return;
        }
        let mut any_inside = false;
        for (zindex, ord_entity) in order.map.iter().rev() {
            if let Ok((entity, mut interactable, transform)) = q_interactable.get_mut(*ord_entity) {
                info!("Ordering entity {:?} with zind {}", ord_entity, zindex);
                let pos = Vec2::new(transform.translation.x, transform.translation.y);
                let maybe_inside = pos.distance(mouse.world_pos) <= interactable.min_dist;
                let inside_fn = interactable.mouse_inside.as_ref().unwrap();
                if order.ui_blocking.is_none() && maybe_inside && (inside_fn)(&pos, &*mouse) {
                    if let Enabled = interactable.state {
                        any_inside = true;
                        order.current = Some(entity);
                        break;
                    };
                }
            }
        }
        if !any_inside {
            order.current = None;
        }
    }
}

pub fn interactable_handling(
    mut commands: Commands,
    mouse: Res<MouseState>,
    order: Res<InteractableOrder>,
    mut er_mouseinput: EventReader<MouseButtonInput>,
    q_interact: Query<(&InteractableObject, &ClickHandlers)>,
) {
    let mut peekable = er_mouseinput.iter().peekable();
    if order.current.is_none()
    || peekable.peek().is_none() {
        return;
    }
    let maybe_query = order.current.as_ref().and_then(|current| {
        q_interact.get(*current).ok()
    });
    // this interactable doesn't have any click handlers for some reason
    if maybe_query.is_none() {
        return;
    }
    let (interactable, handlers) = maybe_query.unwrap();
    for e in peekable.into_iter() {
        if let ElementState::Pressed = e.state {
            continue;
        }
        match e.button {
            MouseButton::Left => {
                if let Some(handler) = handlers.left.as_ref() {
                    (handler)(&mut commands, &*mouse);
                }
            },
            MouseButton::Right => {
                if let Some(handler) = handlers.right.as_ref() {
                    (handler)(&mut commands, &*mouse);
                }
            },
            MouseButton::Middle => {

            },
            _ => {}
        };
    }
}

pub fn make_appearance_interactive(
    mut commands: Commands,
    q_appearance: Query<(Entity, &Appearance, &Transform, &Sprite), Without<InteractableObject>>,
) {
    for (entity, _, _, sprite) in q_appearance.iter() {
        let size = sprite.size;
        // wait until it has a size...
        if size.x < f32::EPSILON && size.y < f32::EPSILON {
            info!("Sprite of {:?} is unsized - waiting...", entity);
            continue;
        }
        info!("Sizing sprite of {:?}!", entity);
        let mut max_dim: f32;
        if size.y > size.x {
            max_dim = size.y / 2.0;
        } else {
            max_dim = size.x / 2.0;
        }
        commands.entity(entity)
            .insert(InteractableObject {
                min_dist: max_dim,
                mouse_inside: Some(Box::new(move |pos: &Vec2, mouse: &MouseState| -> bool {
                    let diff = *pos - mouse.world_pos;
                    diff.x <= size.x / 2.0
                    && diff.y <= size.y / 2.0
                })),
                ..Default::default()
            })
            ;
    }
}