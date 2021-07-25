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

pub struct FuckYou;

pub fn display_navmesh_system(
    mut commands: Commands,
    q_navmesh: Query<(Entity, &NavMesh), Added<NavMesh>>,
    q_deleteme: Query<&FuckYou>,
) {
    for (entity, navmesh) in q_navmesh.iter() {
        let path = path_bundle(navmesh.edges());
        commands.entity(entity)
            .insert_bundle(path)
            .insert(FuckYou)
            ;
    }
}

fn path_bundle(iter: impl Iterator<Item=[Vec2; 2]>) -> ShapeBundle {
    let mut path_builder = PathBuilder::new();
    for edge in iter {
        info!("Building path from {} to {}", edge[0], edge[1]);
        path_builder.move_to(edge[0]);
        path_builder.line_to(edge[1]);
    }
    ShapeBundle {
        path: path_builder.build(),
        mode: DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(2.0),
        },
        colors: ShapeColors::outlined(Color::rgba(0.0, 0.3, 0.75, 0.25), Color::BLUE),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    }
}