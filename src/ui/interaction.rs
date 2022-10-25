use crate::{
    input::*,
    utils::{
        // entities::do_for_children,
        geometry::*,
    },
};
use bevy::{
    input::{mouse::MouseButtonInput},
    prelude::*,
};

#[derive(Default)]
pub struct InteractableOrder {
    pub ui_blocking: Option<Entity>,
    pub current: Option<(Entity, f32)>,
}

#[derive(Component, PartialEq, Eq)]
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

#[derive(Component, Default)]
pub struct ClickHandlers {
    pub left: Option<Box<dyn Fn(&mut Commands, &MouseState) + Send + Sync>>,
    pub right: Option<Box<dyn Fn(&mut Commands, &MouseState) + Send + Sync>>,
    pub middle: Option<Box<dyn Fn(&mut Commands, &MouseState) + Send + Sync>>,
}

#[derive(Component)]
pub struct Highlighted(pub bool);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Polygon {
    pub centroid: Vec2,
    pub points: Vec<Vec2>,
    pub max_dim: f32,
    pub visible: bool,
}

impl Polygon {
    pub fn new(points: Vec<Vec2>) -> Self {
        Polygon {
            centroid: polygon_centroid(&points),
            max_dim: max_polygon_width(&points),
            points,
            visible: false
        }
    }
}

pub fn object_interaction_ordering(
    mut order: ResMut<InteractableOrder>,
    mut q_interaction: Query<(Entity, &Transform, &ObjectInteraction, Option<&mut Highlighted>)>,
    // mut q_interaction_children: Query<(&Parent, &mut Visible)>
) {
    let mut max_z_index = f32::NEG_INFINITY;
    let old = order.current.map(|current| current.0);
    let mut current_highlight: Option<Mut<Highlighted>> = None;
    order.current = q_interaction
        .iter_mut()
        .filter(|(_, _, interaction, _)| **interaction == ObjectInteraction::Inside)
        .fold(None, |acc, (entity, transform, interaction, highlighted)| {
            if transform.translation.z > max_z_index {
                max_z_index = transform.translation.z;
                current_highlight = highlighted;
                return Some((entity, max_z_index));
            }
            acc
        });
    match (old, order.current) {
        (Some(old), Some(current)) => {
            if old != current.0 {
                info!("Replaced {:?} with {:?} in interactables order.", old, current.0);
                set_highlight(old, &mut q_interaction, false);
                set_highlight(current.0, &mut q_interaction, true);
            }
        },
        (None, Some(current)) => {
            info!("New current in order: {:?}", current.0);
            set_highlight(current.0, &mut q_interaction, true);
        },
        (Some(old), None) => {
            info!("Removed {:?} from order; order current is now blank.", old);
            set_highlight(old, &mut q_interaction, false);
        },
        _ => {}
    }
}

fn set_highlight(e: Entity, q: &mut Query<(Entity, &Transform, &ObjectInteraction, Option<&mut Highlighted>)>, val: bool) -> Option<()> {
    let (_, _, _, old_highlight) = q.get_mut(e).ok()?;
    *old_highlight? = Highlighted(val);
    Some(())
}

// TODO what do about visibility??

// pub fn highlight_system(
//     mut q_highlighted: Query<(&Highlighted, Option<&Children>, Option<&mut Visible>), (Without<Parent>, Changed<Highlighted>)>,
//     mut q_visible_children: Query<(&Parent, &mut Visible)>
// ) {
//     for (highlight, maybe_children, maybe_visible) in q_highlighted.iter_mut() {
//         let is_visible = highlight.0;
//         if let Some(mut visible) = maybe_visible {
//             visible.is_visible = is_visible;
//             visible.is_transparent = !is_visible;
//         }
//         set_children_visibility(maybe_children, &mut q_visible_children, is_visible);
//     }
// }

// fn set_children_visibility(children: Option<&Children>, q: &mut Query<(&Parent, &mut Visible)>, is_visible: bool) -> Option<()> {
//     for child in children?.iter() {
//         let (_, mut visible) = q.get_mut(*child).ok()?;
//         visible.is_visible = is_visible;
//         visible.is_transparent = !is_visible;
//     }
//     Some(())
// }

pub fn object_interaction_handling(
    mut commands: Commands,
    mouse: Res<MouseState>,
    order: Res<InteractableOrder>,
    mouse_button: Res<Input<MouseButton>>,
    q_interact: Query<(&ObjectInteraction, &ClickHandlers)>,
) {
    if order.current.is_none() {
        return;
    }
    let maybe_query = order
        .current
        .as_ref()
        .and_then(|current| q_interact.get(current.0).ok());
    if let Some((interactable, handlers)) = maybe_query {
        if mouse_button.just_released(MouseButton::Left) {
            if let Some(handler) = handlers.left.as_ref() {
                (handler)(&mut commands, &mouse);
            }
        } else if mouse_button.just_released(MouseButton::Right) {
            if let Some(handler) = handlers.right.as_ref() {
                info!("starting right click handler");
                (handler)(&mut commands, &mouse);
            }
        }
    }
}

pub fn polygon_interact_system(
    order: Res<InteractableOrder>,
    mouse: Res<MouseState>,
    mut er_cursor: EventReader<CursorMoved>,
    mut q_polygon: Query<(Entity, &Polygon, &mut ObjectInteraction)>,
    // q_polygon_vis: Query<(&Polygon, &Children), Changed<Polygon>>,
    // mut q_polygon_children: Query<(&Parent)>,
) {
    for e in er_cursor.iter() {
        for (entity, polygon, mut interaction) in q_polygon.iter_mut() {
            let maybe_inside = polygon.centroid.distance(mouse.world_pos) <= polygon.max_dim;
            if order.ui_blocking.is_none()
                && maybe_inside 
                && point_inside_polygon(&mouse.world_pos, &polygon.points)
            {
                *interaction = ObjectInteraction::Inside;
            } else {
                *interaction = ObjectInteraction::Outside;
            }
        }
    }
     // TODO these used to have visibility on them, what do now?
    // for (parent, children) in q_polygon_vis.iter() {
    //     let is_visible = parent.visible;
    //     for entity in children.iter() {
    //         if let Ok((_, mut visible)) = q_polygon_children.get_mut(*entity) {
    //             visible.is_visible = is_visible;
    //             visible.is_transparent = !is_visible;
    //         }
    //     }
    // }
}

pub fn capture_interactions(
    mut order: ResMut<InteractableOrder>,
    mut er_mousebutton: EventReader<MouseButtonInput>,
    mut er_mousemove: EventReader<CursorMoved>,
    q_interaction: Query<(Entity, &Interaction)>,
) {
    use Interaction::*;
    if er_mousebutton.iter().next().is_none() && er_mousemove.iter().next().is_none() {
        return;
    }
    for (entity, interact) in q_interaction.iter() {
        match *interact {
            Clicked | Hovered => {
                order.ui_blocking = Some(entity);
                return;
            }
            _ => {}
        };
    }
    order.ui_blocking = Option::None;
}

// normal ui click handling
pub fn interaction_with_handlers(
    mut commands: Commands,
    mouse: Res<MouseState>,
    mouse_input: Res<Input<MouseButton>>,
    q_buttons: Query<(&Interaction, &ClickHandlers), Changed<Interaction>>,
) {
    for (interaction, handlers) in q_buttons.iter() {
        if let Interaction::Hovered = interaction {
            for button in mouse_input.get_just_released() {
                match button {
                    MouseButton::Left => {
                        if let Some(handler) = handlers.left.as_ref() {
                            (handler)(&mut commands, &mouse);
                        }
                    }
                    MouseButton::Right => {
                        if let Some(handler) = handlers.right.as_ref() {
                            (handler)(&mut commands, &mouse);
                        }
                    }
                    MouseButton::Middle => {
                        if let Some(handler) = handlers.middle.as_ref() {
                            (handler)(&mut commands, &mouse);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
