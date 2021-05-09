use bevy::{
    prelude::*,
};
use bevy_prototype_lyon::prelude::*;

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
        points,
        closed: true
    };
    
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        ShapeColors::outlined(Color::TEAL, Color::BLACK),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(5.0),
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(ShapePlugin)
            .add_startup_system(setup.system())
        ;
    }
}