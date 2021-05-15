use bevy::{
    prelude::*,
    input::{
        ElementState,
        mouse::{MouseButtonInput},
    },
};

// systems relating to showing UI elements, views on objects, etc
pub struct InteractablePolygon {
    pub points: Vec<Vec2>,
}

pub struct ContextMenuItem {
    pub label: String,
    pub event_tag: String,
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

fn popup_system(
    mut commands: Commands,
    mut er_mouse: EventReader<MouseButtonInput>,
    q_popup: Query<(&Node, &Interaction), (With<Popup>)>,
) {
    if er_mouse.iter().count() == 0 {
        return;
    }
    for (_, interaction) in q_popup.iter() {
        match *interaction {
            Interaction::None => println!("Deleting interactable"),
            _ => {}
        }
    }
}

fn button_system(
    mut commands: Commands,
    mut er_mouse: EventReader<MouseButtonInput>,
    q_popup: Query<(&Button, &Interaction), (Changed<Interaction>, With<Popup>)>,
) {
    for (_, interaction) in q_popup.iter() {
        match *interaction {
            Interaction::Clicked => println!("Button clicked"),
            _ => {}
        }
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .add_system(popup_system.system())
            .add_system(button_system.system())
        ;
    }
}

// UTILITY FNS

pub fn spawn_context_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Rect<Val>,
    // menu: &ContextMenu,
) {
    commands
    .spawn_bundle(ButtonBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            //direction: Direction,
            //align_content: AlignContent,
            //justify_content: JustifyContent,
            position,
            ..Default::default()
        },
        material: materials.add(Color::BLACK.into()),
        ..Default::default()
    })
    .with_children(|parent| {
        for i in 0..5 {
            parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(75.0), Val::Px(26.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: Rect::all(Val::Px(3.0)),
                    ..Default::default()
                },
                focus_policy: bevy::ui::FocusPolicy::Pass,
                material: materials.add(Color::GRAY.into()),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                            font_size: 16.0,
                            color: Color::BLACK,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            });
        }
    })
    .insert(Popup)
    ;
}