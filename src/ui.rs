use std::{
    marker::PhantomData,
    sync::Arc,
};
use bevy::{ecs::component::Component, input::{mouse::{self, MouseButtonInput}}, prelude::*};
use crate::{
    input::{MouseState,},
    utils::{
        entities::{children_match_query,},
    },
    game::*,
};

mod interaction;
pub use interaction::*;

// systems relating to showing UI elements, views on objects, etc
pub struct Polygon {
    pub points: Arc<Vec<Vec2>>,
}

pub struct ContextMenuItem {
    pub label: String,
    pub handlers: Option<ClickHandlers>,
}

pub struct ContextMenuSpawn {
    pub pos: Vec2,
    pub items: Vec<ContextMenuItem>,
}

pub struct ButtonStyle {
    pub color_normal: Handle<ColorMaterial>,
    pub color_hovered: Handle<ColorMaterial>,
    pub color_clicked: Handle<ColorMaterial>,
    pub text_style: TextStyle,
}

impl FromWorld for ButtonStyle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let font_handle: Handle<Font> = asset_server.load("fonts/OpenSans-Regular.ttf");
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonStyle {
            color_normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            color_hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            color_clicked: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
            text_style: TextStyle {
                font: font_handle,
                font_size: 16.0,
                color: Color::WHITE,
            }
        }
    }
}

// anything with this guy that is a ui element will be closed if clicked outside
pub struct Popup;

#[derive(Default,)]
pub struct ClickHandlers {
    pub left: Option<Box<dyn Fn(&mut Commands, &MouseState) + Send + Sync>>,
    pub right: Option<Box<dyn Fn(&mut Commands, &MouseState) + Send + Sync>>,
    pub middle: Option<Box<dyn Fn(&mut Commands, &MouseState) + Send + Sync>>,
}

pub struct UiPlugin;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn popup_system(
    mut commands: Commands,
    mut er_mouse: EventReader<MouseButtonInput>,
    q_menu: Query<(Entity, &Node, &Children), With<Popup>>,
    q_buttons: Query<(&Button), Changed<Interaction>>,
) {
    if er_mouse.iter().count() == 0 {
        return;
    }
    for (entity, _, children) in q_menu.iter() {
        if !children_match_query(children, &q_buttons) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn capture_interactions(
    mut order: ResMut<InteractableOrder>,
    q_interaction: Query<(Entity, &Interaction), Changed<Interaction>>,
) {
    use Interaction::*;
    for (entity, interact) in q_interaction.iter() {
        match *interact {
            Clicked | Hovered => {
                order.ui_blocking = Some(entity);
                break;
            },
            None => {
                if let Some(e) = &order.ui_blocking {
                    if *e == entity {
                        // this was the blocking entity and it's no longer blocking.
                        order.ui_blocking = Option::None;
                    }
                }
            }
        };
    }
}

fn button(
    button_style: Res<ButtonStyle>,
    mut q_buttons: Query<(&Interaction, &mut Handle<ColorMaterial>), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut material) in q_buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_style.color_clicked.clone();
            },
            Interaction::Hovered => {
                *material = button_style.color_hovered.clone();
            },
            Interaction::None => {
                *material = button_style.color_normal.clone();
            }
        }
    }
}

fn interaction_with_handlers(
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
                            (handler)(&mut commands, &*mouse);
                        }
                    },
                    MouseButton::Right => {
                        if let Some(handler) = handlers.right.as_ref() {
                            (handler)(&mut commands, &*mouse);
                        }
                    },
                    MouseButton::Middle => {
                        if let Some(handler) = handlers.middle.as_ref() {
                            (handler)(&mut commands, &*mouse);
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

fn context_menu_spawn(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_style: Res<ButtonStyle>,
    mut q_cmspawn: Query<(Entity, &mut ContextMenuSpawn), Added<ContextMenuSpawn>>,
) {
    for (entity, mut cm) in q_cmspawn.iter_mut() {
        let mut entity_cmds = commands.entity(entity);
        entity_cmds.remove::<ContextMenuSpawn>();
        entity_cmds.insert_bundle(NodeBundle {
            style: Style {
                display: Display::Flex,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                position: Rect{
                    left: Val::Px(cm.pos.x),
                    top: Val::Px(cm.pos.y),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(Color::BLACK.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            for item in cm.items.iter_mut() {
                parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        min_size: Size::new(Val::Px(75.0), Val::Px(26.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: Rect::all(Val::Px(2.0)),
                        padding: Rect::all(Val::Px(3.0)),
                        ..Default::default()
                    },
                    material: button_style.color_normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            item.label.clone(),
                            button_style.text_style.clone(),
                            Default::default(),
                        ),
                        focus_policy: bevy::ui::FocusPolicy::Pass,
                        ..Default::default()
                    });
                })
                .insert(item.handlers.take().unwrap())
                ;
            }
        })
        .insert(Popup);
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<ButtonStyle>()
            .insert_resource(InteractableOrder::default())
            .add_startup_system(setup.system())
            .add_system(button.system())
            .add_system(interaction_with_handlers.system())
            .add_system(popup_system.system())
            .add_system(context_menu_spawn.system())
            .add_system(capture_interactions.system())
            .add_system(interactable_zindex.system())
            .add_system(interactable_zindex_change.system())
            .add_system(interactable_mouse_inside.system())
            .add_system(interactable_handling.system())
            .add_system(make_appearance_interactive.system())
        ;
    }
}

// pub fn create_context_menu(
//     commands: &mut Commands,
//     bg_color: &Handle<ColorMaterial>,
//     button_mat: &Handle<ColorMaterial>,
//     text_style: &TextStyle,
//     pos: &Vec2,
//     items: &mut [ContextMenuItem],
// ) {
//     commands.spawn_bundle(NodeBundle {
//         style: Style {
//             display: Display::Flex,
//             position_type: PositionType::Absolute,
//             flex_direction: FlexDirection::Column,
//             position: Rect{
//                 left: Val::Px(pos.x),
//                 top: Val::Px(pos.y),
//                 ..Default::default()
//             },
//             ..Default::default()
//         },
//         material: bg_color.clone(),
//         ..Default::default()
//     })
//     .with_children(|parent| {
//         for item in items.iter_mut() {
//             parent
//             .spawn_bundle(ButtonBundle {
//                 style: Style {
//                     min_size: Size::new(Val::Px(75.0), Val::Px(26.0)),
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::Center,
//                     margin: Rect::all(Val::Px(2.0)),
//                     padding: Rect::all(Val::Px(3.0)),
//                     ..Default::default()
//                 },
//                 material: button_mat.clone(),
//                 ..Default::default()
//             })
//             .with_children(|parent| {
//                 parent.spawn_bundle(TextBundle {
//                     text: Text::with_section(
//                         item.label.clone(),
//                         text_style.clone(),
//                         Default::default(),
//                     ),
//                     focus_policy: bevy::ui::FocusPolicy::Pass,
//                     ..Default::default()
//                 });
//             })
//             .insert(item.handlers.take().unwrap())
//             ;
//         }
//     })
//     .insert(Popup);
// }