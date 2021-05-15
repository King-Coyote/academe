use bevy::{
    prelude::*,
    input::{
        ElementState,
        mouse::{MouseButtonInput},
    },
};
use crate::{
    input::{MouseState,},
    utils::entities::{children_match_query,},
};

// systems relating to showing UI elements, views on objects, etc
pub struct InteractablePolygon {
    pub points: Vec<Vec2>,
}

pub struct ContextMenuItem {
    pub label: String,
    pub event_tag: String,
}

struct Durr;

#[derive(Clone)]
pub struct ContextMenuButtonEvent {
    pub tag: String,
    pub mouse_snapshot: MouseState, // where the CM was opened, not where the button was clicked
}

// TODO make macro for instantiating this
pub struct ContextMenu(pub Vec<ContextMenuItem>);

struct Popup;

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

// fn popup_system(
//     mut commands: Commands,
//     mut er_mouse: EventReader<MouseButtonInput>,
//     q_popup: Query<(Entity, &Node, &Interaction), (With<Popup>)>,
// ) {
//     if er_mouse.iter().count() == 0 {
//         return;
//     }
//     for (entity, _, interaction) in q_popup.iter() {
//         if let Interaction::None = *interaction {
//             commands.entity(entity).despawn_recursive();
//         }
//     }
// }

fn popup_system(
    mut commands: Commands,
    mut er_mouse: EventReader<MouseButtonInput>,
    mut ew_cmbutton: EventWriter<ContextMenuButtonEvent>,
    q_menu: Query<(Entity, &Node, &Children), With<Popup>>,
    q_buttons: Query<(&Button, &ContextMenuButtonEvent), Changed<Interaction>>
) {
    if er_mouse.iter().count() == 0 {
        return;
    }
    for (entity, _, children) in q_menu.iter() {
        println!("durr");
        if !children_match_query(children, &q_buttons) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn context_menu_button(

) {
    
}

fn context_button_clicked(
    mut er_cmbutton: EventReader<ContextMenuButtonEvent>,
) {
    for e in er_cmbutton.iter() {
        let wp = e.mouse_snapshot.world_pos;
        println!("Button clicked for world position {}", wp);
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<ContextMenuButtonEvent>()
            .add_startup_system(setup.system())
            .add_system(popup_system.system())
            // .add_system(context_menu_button_system.system())
            .add_system(context_button_clicked.system())
        ;
    }
}

// UTILITY FNS

pub fn spawn_context_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mouse: MouseState,
    menu: &ContextMenu,
) {
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
                    size: Size::new(Val::Px(75.0), Val::Px(26.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: Rect::all(Val::Px(3.0)),
                    ..Default::default()
                },
                material: materials.add(Color::GRAY.into()),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        item.label.clone(),
                        TextStyle {
                            font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    focus_policy: bevy::ui::FocusPolicy::Pass,
                    ..Default::default()
                });
            })
            .insert(Durr)
            .insert(ContextMenuButtonEvent {
                tag: item.event_tag.clone(),
                mouse_snapshot: mouse.clone(),
            });
        }
    })
    .insert(Popup)
    ;
}