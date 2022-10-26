use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// for debug displays of things, like shapes, polygons, navmeshes, etc

#[derive(Component)]
pub struct DebugCircleSpawn {
    pub radius: f32,
    pub color: Color,
    pub center: Vec2,
}

#[derive(Component)]
pub struct DebugLineSpawn {
    pub origin: Vec2,
    pub dest: Vec2,
    pub color: Color,
    pub thickness: f32,
}

pub fn spawn_debug_ui(
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    q_lines: Query<(Entity, &DebugLineSpawn), Added<DebugLineSpawn>>,
    q_circle: Query<(Entity, &DebugCircleSpawn), Added<DebugCircleSpawn>>,
) {
    // TODO UPDATE
    // for (entity, line) in q_lines.iter() {
    //     commands
    //         .entity(entity)
    //         .remove::<DebugLineSpawn>()
    //         .insert_bundle(GeometryBuilder::build_as(
    //             &shapes::Line(line.origin, line.dest),
    //             DrawMode::Outlined {
    //                 fill_mode: FillMode {
    //                     color: Color::RED,
    //                     ..Default::default()
    //                 },
    //                 outline_mode: StrokeMode {
    //                     color: Color::RED,
    //                     options: StrokeOptions {
    //                         line_width: 1.0,
    //                         ..Default::default()
    //                     },
    //                 }
    //             },
    //             Transform::from_xyz(0.0, 0.0, 0.0),
    //         ));
    // }
    // for (entity, circle) in q_circle.iter() {
    //     commands
    //         .entity(entity)
    //         .remove::<DebugCircleSpawn>()
    //         .insert_bundle(GeometryBuilder::build_as(
    //             &shapes::Circle {
    //                 radius: circle.radius,
    //                 center: circle.center,
    //             },
    //             DrawMode::Outlined {
    //                 fill_mode: FillMode {
    //                     color: Color::RED,
    //                     ..Default::default()
    //                 },
    //                 outline_mode: StrokeMode {
    //                     color: Color::RED,
    //                     options: StrokeOptions {
    //                         line_width: 1.0,
    //                         ..Default::default()
    //                     },
    //                 }
    //             },
    //             Transform::from_xyz(0.0, 0.0, 0.0),
    //         ));
    // }
}
