use bevy::prelude::*;
use crate::nav::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;

// for displaying things, etc

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(display_navmesh_system.system())
        ;
    }
}

pub fn display_navmesh_system(
    mut commands: Commands,
    q_navmesh: Query<(Entity, &NavMesh), Added<NavMesh>>,
) {
    for (entity, navmesh) in q_navmesh.iter() {
        commands.entity(entity).with_children(|parent| {
            for triangle in navmesh.interior_triangles() {
                parent.spawn_bundle(triangle_bundle(&triangle));
            }
            for node in navmesh.graph_nodes_iter() {
                parent.spawn_bundle(vertex_bundle(*node, Color::RED));
            }
            let path = navmesh.graph_edges();
            let path_bundle = path_bundle(&path, Color::GREEN);
            parent.spawn_bundle(path_bundle);
        })
        ;
    }
}

pub fn triangle_bundle(t: &[Vec2; 3]) -> ShapeBundle {
    GeometryBuilder::build_as(
        &shapes::Polygon {
            points: t.to_vec(),
            closed: true,
        },
        ShapeColors::outlined(Color::rgba(0.0, 0.3, 0.75, 0.25), Color::BLUE),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(1.0),
        },
        Transform::from_xyz(0.0, 0.0, 1000.0),
    )
}

pub fn path_bundle(
    points: &[(&Vec2, &Vec2)],
    color: Color,
) -> ShapeBundle {
    let mut path_builder = PathBuilder::new();
    for edge in points {
        info!("Building path from {} to {}", edge.0, edge.1);
        path_builder.move_to(*edge.0);
        path_builder.line_to(*edge.1);
    }
    ShapeBundle {
        path: path_builder.build(),
        mode: DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(2.0),
        },
        colors: ShapeColors::outlined(Color::rgba(0.0, 0.3, 0.75, 0.25), color),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    }
}

pub fn vertex_bundle(point: Vec2, color: Color) -> ShapeBundle {
    GeometryBuilder::build_as(
        &shapes::Circle {
            radius: 5.0,
            center: point
        }, 
        ShapeColors::new(color), 
        DrawMode::Fill(FillOptions::default()), 
        Transform::from_xyz(0.0, 0.0, 1000.0)
    )
}