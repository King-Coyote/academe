use bevy::{
    input::{
        ElementState,
        mouse::{MouseButtonInput},
    },
    prelude::*,
};
use bevy_prototype_lyon::prelude::*;
use crate::{
    input::*,
    utils::{point_inside_polygon,},
    ui::*,
};

// curently for rendering spaces and allowing them to be interacted with.
pub struct SpacePlugin;

fn setup(
    mut commands: Commands,
) {
    let points = vec![
        Vec2::new(0.0, 150.0),
        Vec2::new(300.0, 0.0),
        Vec2::new(0.0, -150.0),
        Vec2::new(-300.0, 0.0),
    ];

    let shape = shapes::Polygon {
        points: points.clone(),
        closed: true
    };
    
    commands.spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(Color::TEAL, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(5.0),
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .insert(InteractablePolygon{points})
        .insert(InteractState(InteractStateEnum::Enabled))
        .insert(ContextMenu(vec![
            ContextMenuItem{
                label: "Spawn entity".to_owned(),
                event_tag: "spawn_entity".to_owned()
            },
        ]))
    ;
}

fn polygon_interact_system(
    mut commands: Commands,
    mouse: Res<MouseState>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut er_mousemove: EventReader<CursorMoved>,
    mut er_mouseinput: EventReader<MouseButtonInput>,
    mut q_polygon: Query<(Entity, &InteractablePolygon, &mut InteractState)>,
) {
    use InteractStateEnum::*;
    for e in er_mousemove.iter() {
        for (entity, polygon, mut state) in q_polygon.iter_mut() {
            let inside = point_inside_polygon(&mouse.world_pos, &polygon.points);
            if inside && state.0 == Enabled {
                println!("Hovered over {:?}", entity);
                state.0 = Hovered;
            } else if !inside && state.0 == Hovered {
                println!("Un-hovered {:?}", entity);
                state.0 = Enabled;
            }
        }
    }
    for e in er_mouseinput.iter() {
        for (entity, polygon, mut state) in q_polygon.iter_mut() {
            if e.button != MouseButton::Right {
                continue;
            }
            if state.0 != Hovered && state.0 != Clicked {
                continue;
            }
            if e.state == ElementState::Pressed && state.0 == Hovered {
                println!("Mousedown on {:?}", entity);
                state.0 = Clicked;
            } else if e.state == ElementState::Released && state.0 == Clicked {
                println!("Clicked on {:?}", entity);
                let position = Rect{
                    left: Val::Px(mouse.ui_pos.x),
                    top: Val::Px(mouse.ui_pos.y),
                    ..Default::default()
                };
                spawn_context_menu(&mut commands, &asset_server, &mut materials, position);
                state.0 = Hovered;
            }
        }
    }
}

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(ShapePlugin)
            .add_startup_system(setup.system())
            .add_system(polygon_interact_system.system())
        ;
    }
}