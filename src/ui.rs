use std::{
    marker::PhantomData,
    sync::Arc,
};

use bevy::{
    prelude::*,
    input::{
        ElementState,
        mouse::{MouseButtonInput},
    },
    ecs::component::Component,
};
use crate::{
    input::{MouseState,},
    utils::{
        entities::{children_match_query,},
        geometry::point_inside_polygon,
    },
    game::*,
};

// systems relating to showing UI elements, views on objects, etc
pub struct InteractablePolygon {
    pub points: Vec<Vec2>,
}

pub struct ContextMenuItem {
    pub label: String,
    pub commands: Arc<GameCommandQueue>,
}

pub struct ContextMenu(pub Vec<ContextMenuItem>);

pub struct ButtonStyle {
    color_normal: Handle<ColorMaterial>,
    color_hovered: Handle<ColorMaterial>,
    color_clicked: Handle<ColorMaterial>,
    text_style: TextStyle,
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

pub struct Popup;

// this, when it exists on an entity, will allow certain components of type T to be seen in the ui
pub struct View<T: Component>(pub PhantomData<T>);

#[derive(PartialEq)]
pub enum InteractStateEnum {
    Enabled,
    Clicked,
    Hovered,
    Disabled,
}

#[derive(PartialEq)]
pub struct InteractState(pub InteractStateEnum);

pub struct UiPlugin;

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn popup_system(
    mut commands: Commands,
    mut er_mouse: EventReader<MouseButtonInput>,
    q_menu: Query<(Entity, &Node, &Children), With<Popup>>,
    q_buttons: Query<&Button, Changed<Interaction>>
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

fn button(
    button_style: Res<ButtonStyle>,
    mut q_buttons: Query<(&Interaction, &mut Handle<ColorMaterial>), With<Button>>,
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

fn command_button(
    mut command_queue: ResMut<GameCommandQueue>,
    mut q_buttons: Query<(&Button, &Interaction, &GameCommandQueue), Changed<Interaction>>,
) {
    for (_, interaction, commands) in q_buttons.iter() {
        if let Interaction::Clicked = *interaction {
            command_queue.extend(commands.iter().cloned());
        }
    }
}

fn polygon_interaction(
    mut commands: Commands,
    mouse: Res<MouseState>,
    button_style: Res<ButtonStyle>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut er_mousemove: EventReader<CursorMoved>,
    mut er_mouseinput: EventReader<MouseButtonInput>,
    mut q_polygon: Query<(Entity, &InteractablePolygon, &mut InteractState, &ContextMenu)>,
) {
    use InteractStateEnum::*;
    for e in er_mousemove.iter() {
        for (entity, polygon, mut state, _) in q_polygon.iter_mut() {
            let inside = point_inside_polygon(&mouse.world_pos, &polygon.points);
            if inside && state.0 == Enabled {
                state.0 = Hovered;
            } else if !inside && state.0 == Hovered {
                state.0 = Enabled;
            }
        }
    }
    for e in er_mouseinput.iter() {
        for (entity, polygon, mut state, menu) in q_polygon.iter_mut() {
            if e.button != MouseButton::Right {
                continue;
            }
            if state.0 != Hovered && state.0 != Clicked {
                continue;
            }
            if e.state == ElementState::Pressed && state.0 == Hovered {
                state.0 = Clicked;
            } else if e.state == ElementState::Released && state.0 == Clicked {
                let view: View<ContextMenu> = View(PhantomData);
                commands.entity(entity).insert(view);
                state.0 = Hovered;
            }
        }
    }
}

fn context_menu_view(
    mut commands: Commands,
    mouse: Res<MouseState>,
    button_style: Res<ButtonStyle>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_view: Query<(Entity, &ContextMenu), Added<View<ContextMenu>>>,
) {
    for (entity, menu) in q_view.iter() {
        commands.entity(entity).remove::<View<ContextMenu>>();
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    position: Rect{
                        left: Val::Px(mouse.ui_pos.x),
                        top: Val::Px(mouse.ui_pos.y),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with_children(|parent| {
                for item in menu.0.iter() {
                    parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            min_size: Size::new(Val::Px(75.0), Val::Px(26.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: Rect::all(Val::Px(3.0)),
                            padding: Rect::all(Val::Px(3.0)),
                            ..Default::default()
                        },
                        material: materials.add(Color::GRAY.into()),
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
                    .insert(clone_commands_with_targets(&item.commands, entity, &mouse))
                    ;
                }
            })
            .insert(Popup)
        ;
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<ButtonStyle>()
            .add_startup_system(setup.system())
            .add_system(button.system())
            .add_system(command_button.system())
            .add_system(popup_system.system())
            .add_system(context_menu_view.system())
            .add_system(polygon_interaction.system())
        ;
    }
}

// UTILITY FNS
fn clone_commands_with_targets(
    queue: &GameCommandQueue,
    entity: Entity,
    mouse: &MouseState,
) -> GameCommandQueue {
    use Target::*;
    GameCommandQueue (
        queue.iter().map(|cmd| {
            let new_target: Target = match cmd.target {
                World(_) => World(Some(mouse.world_pos)),
                Screen(_) => Screen(Some(mouse.screen_pos)),
                Entity(_) => Entity(Some(entity)),
            };
            GameCommand {
                target: new_target,
                command: cmd.command.clone(),
                level: cmd.level
            }
        })
        .collect()
    )
}