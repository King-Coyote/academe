use crate::{
    game::*,
    input::MouseState,
    utils::{
        entities::children_match_query,
        geometry::{point_inside_polygon, max_polygon_width, polygon_centroid},
    },
};
use bevy::{
    input::mouse::{MouseButtonInput},
    prelude::*,
};

mod interaction;
pub use interaction::*;
mod editor_interface;
pub use editor_interface::*;
mod debug;
pub use debug::*;

// systems relating to showing UI elements, views on objects, etc
#[derive(Reflect, Default)]
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

pub struct ContextMenuItem {
    pub label: String,
    pub handlers: Option<ClickHandlers>,
}

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
pub struct Popup;

#[derive(Default)]
pub struct ClickHandlers {
    pub left: Option<Box<dyn Fn(&mut Commands, &MouseState) + Send + Sync>>,
    pub right: Option<Box<dyn Fn(&mut Commands, &MouseState) + Send + Sync>>,
    pub middle: Option<Box<dyn Fn(&mut Commands, &MouseState) + Send + Sync>>,
}

pub struct UiPlugin;

fn setup(mut commands: Commands, materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn polygon_interact_system(
    order: Res<InteractableOrder>,
    mouse: Res<MouseState>,
    mut er_cursor: EventReader<CursorMoved>,
    mut q_polygon: Query<(Entity, &Polygon, &mut ObjectInteraction)>,
    q_polygon_vis: Query<(&Polygon, &Children), Changed<Polygon>>,
    mut q_polygon_children: Query<(&Parent, &mut Visible)>,
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
    for (parent, children) in q_polygon_vis.iter() {
        let is_visible = parent.visible;
        for entity in children.iter() {
            if let Ok((_, mut visible)) = q_polygon_children.get_mut(*entity) {
                visible.is_visible = is_visible;
                visible.is_transparent = !is_visible;
            }
        }
    }
}

fn set_polygon_child_visibility(children: &Children, query: &mut Query<(&Parent, &mut Visible)>, is_visible: bool) {

}

fn popup_system(
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

fn capture_interactions(
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

fn button(
    style: Res<MainStyle>,
    mut q_buttons: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in q_buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = style.button.color_clicked.clone();
            }
            Interaction::Hovered => {
                *material = style.button.color_hovered.clone();
            }
            Interaction::None => {
                *material = style.button.color_normal.clone();
            }
        }
    }
}

// normal ui click handling
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
                    }
                    MouseButton::Right => {
                        if let Some(handler) = handlers.right.as_ref() {
                            (handler)(&mut commands, &*mouse);
                        }
                    }
                    MouseButton::Middle => {
                        if let Some(handler) = handlers.middle.as_ref() {
                            (handler)(&mut commands, &*mouse);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn context_menu_spawn(
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
                    position: Rect {
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
                            material: style.button.color_normal.clone(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    item.label.clone(),
                                    style.text.clone(),
                                    Default::default(),
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

// TODO shouldnt this be split up a bit?
impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MainStyle>()
            .insert_resource(InteractableOrder::default())
            .register_type::<Polygon>()
            .add_event::<ObjectHovered>()
            .add_startup_system(setup.system())
            .add_system(polygon_interact_system.system())
            .add_system(button.system())
            .add_system(interaction_with_handlers.system())
            .add_system(popup_system.system())
            .add_system(context_menu_spawn.system())
            .add_system(capture_interactions.system())
            .add_system(object_interaction_ordering.system())
            .add_system(object_interaction_handling.system())
            .add_system(highlight_system.system())
            .add_system(appearance_interact_system.system())
            .add_system(make_appearance_interactive.system())
            .add_system(spawn_debug_ui.system())
            .add_system(save_load.exclusive_system())
            ;
    }
}
