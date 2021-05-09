use bevy::{
    prelude::*,
};
use bevy_prototype_lyon::prelude::*;

// curently for rendering spaces and allowing them to be interacted with.

pub struct SpacePlugin;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    let points = vec![
        Vec2::new(0.0, 150.0),
        Vec2::new(300.0, 0.0),
        Vec2::new(0.0, -150.0),
        Vec2::new(-300.0, 0.0),
    ];

    let shape = shapes::Polygon {
        points: points,
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

fn render_simple_space() {

}

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // .insert_resource(Msaa { samples: 8 })
            .add_plugin(ShapePlugin)
            .add_startup_system(setup.system())
            // .add_system(render_simple_space.system())
        ;
    }
}