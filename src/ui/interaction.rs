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
};

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

pub struct Interactable{
    pub min_dist: f32, // only really look at things inside this distance. For efficiency
    pub state: InteractState,
    pub mouse_inside: Option<Box<dyn Fn(&MouseState) -> bool + Send + Sync>>,
    pub on_click: Option<Box<dyn Fn(&mut Commands, &MouseState) + Send + Sync>>,
    pub on_rightclick: Option<Box<dyn Fn(&mut Commands, &MouseState) + Send + Sync>>,
}

pub struct MouseInside;

impl Default for Interactable {
    fn default() -> Self {
        Interactable {
            min_dist: 0.0,
            state: InteractState::Enabled,
            mouse_inside: None,
            on_click: None,
            on_rightclick: None,
        }
    }
}

#[derive(PartialEq)]
pub struct ZIndex(i32);
impl_deref_mut!(ZIndex, i32);

pub fn interactable_zindex(
    mut commands: Commands,
    mut order: ResMut<InteractableOrder>,
    q_interact_link: Query<(Entity, &Transform), (With<Interactable>, Without<ZIndex>)>,
    mut q_interact_change: Query<(&Transform, &mut ZIndex), (With<Interactable>, Changed<Transform>)>,
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
    q_interact: Query<(Entity, &ZIndex), (With<Interactable>, Changed<ZIndex>)>,
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
    mut q_interactable: Query<(Entity, &mut Interactable, &Transform)>,
) {
    use InteractState::*;
    for e in er_mousemove.iter() {
        for (entity, mut interactable, transform) in q_interactable.iter_mut() {
            let pos = Vec2::new(transform.translation.x, transform.translation.y);
            let maybe_inside = pos.distance(mouse.world_pos) <= interactable.min_dist;
            let inside_fn = interactable.mouse_inside.as_ref().unwrap();
            if order.ui_blocking.is_none() && maybe_inside && (inside_fn)(&*mouse) {
                if let Enabled = interactable.state {
                    interactable.state = InsideBounds;
                };
            } else {
                match interactable.state {
                    Disabled | Enabled => {},
                    _ => interactable.state = Enabled,
                };
                if let Some(current) = order.current {
                    if current == entity {
                        // no longer inside bounds of currently hovered entity
                        order.current = None;
                    }
                }
            }
        }
    }
}

pub fn interactable_capture(
    mut order: ResMut<InteractableOrder>,
    er_mousemove: EventReader<CursorMoved>,
    q_interactable: Query<(Entity, &Interactable), Changed<Interactable>>,
) {
    if order.ui_blocking.is_some() {
        order.current = None;
        return;
    }
    if q_interactable.iter().next().is_none() {
        return;
    }
    let mut new_current: Option<Entity> = None;
    for (_, ordered_entity) in order.map.iter() {
        if let Ok((entity, interact)) = q_interactable.get(*ordered_entity) {
            if let InteractState::InsideBounds = interact.state {
                new_current = Some(entity);
                break;
            }
        }
    }
    if let Some(new) = new_current {
        order.current = Some(new);
    }
}

pub fn interactable_input(
    mut commands: Commands,
    mouse: Res<MouseState>,
    order: Res<InteractableOrder>,
    mut er_mouseinput: EventReader<MouseButtonInput>,
    q_interact: Query<&Interactable>,
) {
    if order.current.is_none() {
        return;
    }
    let current = order.current.as_ref().unwrap();
    let interactable = q_interact.get(*current).unwrap();
    for e in er_mouseinput.iter() {
        if let ElementState::Pressed = e.state {
            continue;
        }
        match e.button {
            MouseButton::Left => {
                // if let Some(handler) = interactable.on_click.as_ref() {
                //     entity_cmds.insert((handler)(&*mouse));
                // }
            },
            MouseButton::Right => {
                if let Some(handler) = interactable.on_rightclick.as_ref() {
                    (handler)(&mut commands, &*mouse);
                }
            },
            MouseButton::Middle => {

            },
            _ => {}
        };
    }
}