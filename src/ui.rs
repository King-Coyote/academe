use std::{
    marker::PhantomData,
    sync::Arc,
};
use bevy::{
    prelude::*,
    input::{
        mouse::{MouseButtonInput},
    },
    ecs::component::Component,
};
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
    pub commands: Arc<GameCommandQueue>,
    pub closing: bool,
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
pub struct ClosingButton;

// this, when it exists on an entity, will allow certain components of type T to be seen in the ui
pub struct View<T: Component>(pub PhantomData<T>);

pub struct UiPlugin;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(25.0), Val::Percent(25.0)),
            position: Rect {
                left: Val::Px(100.0),
                top: Val::Px(100.0),
                bottom: Val::Px(300.0),
                right: Val::Px(300.0),
            },
            ..Default::default()
        },
        material: materials.add(Color::BLACK.into()),
        ..Default::default()
    })
    ;
}

fn popup_system(
    mut commands: Commands,
    mut er_mouse: EventReader<MouseButtonInput>,
    q_menu: Query<(Entity, &Node, &Children), With<Popup>>,
    q_buttons: Query<(&Button, &Option<ClosingButton>), Changed<Interaction>>,
) {
    if er_mouse.iter().count() == 0 {
        return;
    }
    for (entity, _, children) in q_menu.iter() {
        if !children_match_query(children, &q_buttons) 
        || q_buttons.iter().any(|(_, closing)| closing.is_some()) {
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
                println!("Blocking with entity {:?}", entity);
                break;
            },
            None => {
                if let Some(e) = &order.ui_blocking {
                    if *e == entity {
                        // this was the blocking entity and it's no longer blocking.
                        println!("Entity {:?} no longer blocking", entity);
                        order.ui_blocking = Option::None;
                    }
                }
            }
        };
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
    q_buttons: Query<(&Button, &Interaction, &GameCommandQueue), Changed<Interaction>>,
) {
    for (_, interaction, commands) in q_buttons.iter() {
        if let Interaction::Clicked = *interaction {
            command_queue.extend(commands.iter().cloned());
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
                material: materials.add(Color::BLACK.into()),
                ..Default::default()
            })
            .with_children(|parent| {
                for item in menu.0.iter() {
                    let closing = item.closing.then(|| ClosingButton);
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
                    .insert(closing)
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
            .insert_resource(InteractableOrder::default())
            .add_startup_system(setup.system())
            .add_system(button.system())
            .add_system(command_button.system())
            .add_system(popup_system.system())
            .add_system(context_menu_view.system())
            .add_system(capture_interactions.system())
            .add_system(interactable_zindex.system())
            .add_system(interactable_zindex_change.system())
            .add_system(interactable_mouse_inside.system())
            .add_system(interactable_capture.system())
            .add_system(interactable_input.system())
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
                LastCreated => LastCreated,
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