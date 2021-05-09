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
};

// curently for rendering spaces and allowing them to be interacted with.
pub struct SpacePlugin;

pub struct InteractablePolygon {
    points: Vec<Vec2>,
}

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
    ;
}

fn interactable_polygon_system(
    mouse: Res<MouseState>,
    mut er_mouse: EventReader<MouseButtonInput>,
    q_polygon: Query<&InteractablePolygon>,
) {
    for polygon in q_polygon.iter() {
        for e in er_mouse.iter() {

        }
    }
}

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(ShapePlugin)
            .add_startup_system(setup.system())
            .add_system(interactable_polygon_system.system())
        ;
    }
}