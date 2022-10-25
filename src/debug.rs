use bevy::prelude::*;
use crate::nav::*;
// use bevy_prototype_lyon::prelude::*;
// use bevy_prototype_lyon::entity::ShapeBundle;

// for displaying things, etc

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(display_navmesh_system)
        ;
    }
}

pub fn display_navmesh_system(
    // mut commands: Commands,
    q_navmesh: Query<(Entity, &NavMesh), Added<NavMesh>>,
) {
    // TODO UPDATE
    // for (entity, navmesh) in q_navmesh.iter() {
    //     commands.entity(entity).with_children(|parent| {
    //         for triangle in navmesh.interior_triangles() {
    //             parent.spawn_bundle(triangle_bundle(&triangle));
    //         }
    //         for node in navmesh.graph_nodes_iter() {
    //             parent.spawn_bundle(vertex_bundle(*node, Color::RED));
    //         }
    //         let path = navmesh.graph_edges();
    //         let path_bundle = path_bundle(&path, Color::GREEN);
    //         parent.spawn_bundle(path_bundle);
    //     })
    //     ;
    // }
}

// pub fn triangle_bundle(t: &[Vec2; 3]) -> ShapeBundle {
//     GeometryBuilder::build_as(
//         &shapes::Polygon {
//             points: t.to_vec(),
//             closed: true,
//         },
//         DrawMode::Outlined {
//             fill_mode: FillMode {
//                 color: Color::rgba(0.0, 0.3, 0.75, 0.25),
//                 ..Default::default()
//             },
//             outline_mode: StrokeMode {
//                 color: Color::BLUE,
//                 options: StrokeOptions {
//                     line_width: 1.0,
//                     ..Default::default()
//                 },
//             }
//         },
//         Transform::from_xyz(0.0, 0.0, 1000.0),
//     )
// }

// pub fn path_bundle(
//     points: &[(&Vec2, &Vec2)],
//     color: Color,
// ) -> ShapeBundle {
//     let mut path_builder = PathBuilder::new();
//     for edge in points {
//         path_builder.move_to(*edge.0);
//         path_builder.line_to(*edge.1);
//     }
//     ShapeBundle {
//         path: path_builder.build(),
//         mode: DrawMode::Outlined {
//             fill_mode: FillMode {
//                 color: Color::rgba(0.0, 0.3, 0.75, 0.25),
//                 ..Default::default()
//             },
//             outline_mode: StrokeMode {
//                 color: Color::BLUE,
//                 options: StrokeOptions {
//                     line_width: 2.0,
//                     ..Default::default()
//                 },
//             }
//         },
//         transform: Transform::from_xyz(0.0, 0.0, 0.0),
//         ..Default::default()
//     }
// }

// pub fn vertex_bundle(point: Vec2, color: Color) -> ShapeBundle {
//     GeometryBuilder::build_as(
//         &shapes::Circle {
//             radius: 5.0,
//             center: point
//         }, 
//         DrawMode::Fill(FillMode {
//             color: color,
//             ..Default::default()
//         }), 
//         Transform::from_xyz(0.0, 0.0, 1000.0)
//     )
// }