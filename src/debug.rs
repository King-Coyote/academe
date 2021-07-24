use bevy::prelude::*;
use crate::nav::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;

// for displaying things, etc

pub fn display_navmesh_system(
    commands: Commands,
    q_navmesh: Query<(Entity, &NavMesh), Added<NavMesh>>,
) {
    for (entity, navmesh) in q_navmesh.iter() {
        for edge in navmesh.edges() {

        }
    }
}

// gives a SB for a line from a to b
fn polygon(
    a: Vec2, 
    b: Vec2
) -> ShapeBundle {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(a);
    path_builder.line_to(b);
    ShapeBundle {
        path: path_builder.build(),
        mode: DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(1.0),
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    }
    // ShapeColors::outlined(Color::rgba(0.0, 0.3, 0.75, 0.25), Color::BLUE),
    // DrawMode::Outlined {
    //     fill_options: FillOptions::default(),
    //     outline_options: StrokeOptions::default().with_line_width(1.0),
    // },
    // Transform::from_xyz(0.0, 0.0, 0.0),
}

// fn spawn_line(
//     mut commands: &mut Commands,
//     materials: &Res<Materials>,
//     meshes: &mut ResMut<Assets<Mesh>>, 
//     a: [CoordNum; 2],
//     b: [CoordNum; 2]
// ) {
//     commands
//     .spawn(line_sprite(
//         &materials,
//         meshes,
//         a,
//         b
//     ))
//     .with(DebugLine)
//     ;
// }

// fn spawn_circle(
//     mut commands: &mut Commands,
//     material: Handle<ColorMaterial>,
//     meshes: &mut ResMut<Assets<Mesh>>, 
//     pos: [CoordNum; 2],
//     r: f32,
// ) {
//     commands.spawn(
//         primitive(
//             material,
//             meshes,
//             ShapeType::Circle(r),
//             TessellationMode::Fill(&FillOptions::default()),
//             Vec3::new(pos[0] as f32, pos[1] as f32, 0.).into(),
//         )
//     )
//     .with(DebugCircle)
//     ;
// }