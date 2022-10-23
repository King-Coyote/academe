use bevy::{
    prelude::*,
    input::mouse::{MouseButtonInput},
};
use crate::{
    utils::entities::children_match_query,
    ClickHandlers,
};

pub struct ContextMenuItem {
    pub label: String,
    pub handlers: Option<ClickHandlers>,
}

#[derive(Component)]
pub struct ContextMenuSpawn {
    pub pos: Vec2,
    pub items: Vec<ContextMenuItem>,
}

pub struct MainStyle {
    panel: PanelStyle,
    button: ButtonStyle,
    text: TextStyle,
}

impl FromWorld for MainStyle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let font_handle: Handle<Font> = asset_server.load("fonts/OpenSans-Regular.ttf");
        let materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        MainStyle {
            panel: PanelStyle::from_world(world),
            button: ButtonStyle::from_world(world),
            text: TextStyle {
                font: font_handle,
                font_size: 16.0,
                color: Color::WHITE,
            },
        }
    }
}

pub struct ButtonStyle {
    pub color_normal: Handle<ColorMaterial>,
    pub color_hovered: Handle<ColorMaterial>,
    pub color_clicked: Handle<ColorMaterial>,
}

const COLOR_BUTTON_NORMAL: Color = Color::rgb(0.15, 0.15, 0.15);
const COLOR_BUTTON_HOVERED: Color = Color::rgb(0.25, 0.25, 0.25);
const COLOR_BUTTON_PRESSED: Color = Color::rgb(0.25, 0.40, 0.55);

impl FromWorld for ButtonStyle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonStyle {
            color_normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            color_hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            color_clicked: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

pub struct PanelStyle {
    pub color: Handle<ColorMaterial>,
    pub clear: Handle<ColorMaterial>,
}

impl FromWorld for PanelStyle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        PanelStyle {
            color: materials.add(Color::GRAY.into()),
            clear: materials.add(Color::NONE.into()),
        }
    }
}

// anything with this guy that is a ui element will be closed if clicked outside
#[derive(Component)]
pub struct Popup;

pub fn popup_system(
    mut commands: Commands,
    mut er_mouse: EventReader<MouseButtonInput>,
    mouse_input: Res<Input<MouseButton>>,
    q_menu: Query<(Entity, &Node, &Children), With<Popup>>,
    q_not_buttons: Query<&Node, (Changed<Interaction>, Without<Button>)>,
    q_buttons: Query<&Button, Changed<Interaction>>,
) {
    if mouse_input.get_just_released().is_empty() || er_mouse.iter().count() == 0 {
        return;
    }
    for (entity, _, children) in q_menu.iter() {
        if children_match_query(children, &q_buttons)
            || !children_match_query(children, &q_not_buttons)
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn button(
    style: Res<MainStyle>,
    mut q_buttons: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in q_buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = COLOR_BUTTON_PRESSED.into();
            }
            Interaction::Hovered => {
                *color = COLOR_BUTTON_HOVERED.into();
            }
            Interaction::None => {
                *color = COLOR_BUTTON_NORMAL.into();
            }
        }
    }
}

pub fn context_menu_spawn(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    style: Res<MainStyle>,
    mut q_cmspawn: Query<(Entity, &mut ContextMenuSpawn), Added<ContextMenuSpawn>>,
) {
    for (entity, mut cm) in q_cmspawn.iter_mut() {
        let mut entity_cmds = commands.entity(entity);
        entity_cmds.remove::<ContextMenuSpawn>();
        entity_cmds
            .insert_bundle(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    position: UiRect {
                        left: Val::Px(cm.pos.x),
                        top: Val::Px(cm.pos.y),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                color: UiColor(Color::BLACK),
                ..Default::default()
            })
            .with_children(|parent| {
                for item in cm.items.iter_mut().rev() {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                min_size: Size::new(Val::Px(75.0), Val::Px(26.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(2.0)),
                                padding: UiRect::all(Val::Px(3.0)),
                                ..Default::default()
                            },
                            color: UiColor(Color::GRAY),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::from_section(
                                    item.label.clone(),
                                    style.text.clone()
                                ),
                                focus_policy: bevy::ui::FocusPolicy::Pass,
                                ..Default::default()
                            });
                        })
                        .insert(item.handlers.take().unwrap());
                }
            })
            .insert(Popup);
    }
}
