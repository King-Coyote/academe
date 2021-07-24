/*
use navigation::prelude::*;
use bevy::{
    prelude::*,
    input::{
        mouse::MouseButtonInput,
        ElementState,
    },
    ecs::SystemStage,
};
use bevy_prototype_lyon::prelude::*;
use crate::main_cam::*;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(startup.system())
        .add_system(add_navpoint_system.system())
        .add_stage_after(stage::UPDATE, "changes", SystemStage::parallel())
        .add_system_to_stage("changes", changed_navmesh_system.system())
        ;
    }
}

struct Materials {
    line_mat: Handle<ColorMaterial>,
    node_mat: Handle<ColorMaterial>,
    vert_mat: Handle<ColorMaterial>,
}

struct DebugLine;
struct DebugCircle;
struct DebugLastPoint([CoordNum; 2]);

const POINTS: [[CoordNum; 2]; 6] = [
    [0., 0.],
    [0., 200.],
    [200., 200.],
    [400., 200.],
    [400., 0.],
    [200., 0.],
];

fn startup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Materials {
        line_mat: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        node_mat: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        vert_mat: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
    });
    // let mut mesh = NavMesh::from_points(&POINTS);
    // mesh.bake();

    // one spawn for each rect, each with the correct size and translation
    commands
        .spawn(Camera2dBundle::default())
            .with(MainCamera)
        .spawn((NavMesh::new(),))
            .with(DebugLastPoint([0.,0.]))
        ;

}

fn add_navpoint_system(
    mut commands: &mut Commands,
    materials: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mouse_state: ResMut<MouseState>,
    mb_events: Res<Events<MouseButtonInput>>,
    keys: Res<Input<KeyCode>>,
    mut nav_q: Query<&mut NavMesh>,
) {
    for event in mouse_state.mousebutton_reader.iter(&mb_events) {
        if event.state == ElementState::Pressed {
            break;
        }
        match event.button {
            MouseButton::Left => {
                for mut navmesh in nav_q.iter_mut() {
                    if keys.pressed(KeyCode::LControl) {
                        navmesh.clear();
                    } else {
                        let pos = [
                            mouse_state.world_pos.x as CoordNum,
                            mouse_state.world_pos.y as CoordNum,
                        ];
                        navmesh.add_point(pos.clone());
                        println!("Added point {:?}", pos);
                    }
                }
            },
            MouseButton::Right => {
                for mut navmesh in nav_q.iter_mut() {
                    if navmesh.is_empty() {
                        continue;
                    }
                    println!("baking navmesh....");
                    navmesh.bake();
                }
            },
            _ => {}
        }
    }
}


fn changed_navmesh_system(
    mut commands: &mut Commands,
    materials: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut navmesh_q: Query<(&NavMesh), (Changed<NavMesh>)>,
    mut line_q: Query<(&DebugLine, Entity)>,
    mut circle_q: Query<(&DebugCircle, Entity)>,
) {
    for nm in navmesh_q.iter() {
        for (_, e) in line_q.iter_mut() {
            commands.despawn(e);
        }
        for (_, e) in circle_q.iter_mut() {
            commands.despawn(e);
        }
        // the nm has changed:
        // therefore, we want to despawn all the sprites we had
        // and respawn them anew as appropriate
        // ie the stupidest, least performant way possible :)
        let edges = nm.edges();
        println!("drawing {} edges", edges.len());
        for line in edges.iter() {
            spawn_line(commands, &materials, &mut meshes, line[0], line[1]);
            spawn_circle(commands, materials.vert_mat.clone(), &mut meshes, line[0], 5.0);
        }
        for centroid in nm.deleteme_centroids().iter() {
            spawn_circle(commands, materials.vert_mat.clone(), &mut meshes, *centroid, 5.0);
        }
    }
}

// utility

// gives a SB for a line from a to b
fn line_sprite(
    materials: &Res<Materials>,
    meshes: &mut ResMut<Assets<Mesh>>, 
    a: [CoordNum; 2], 
    b: [CoordNum; 2]
) -> SpriteBundle {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(point(a[0] as f32, a[1] as f32));
    path_builder.line_to(point(b[0] as f32, b[1] as f32));
    let path = path_builder.build();
    path.stroke(
        materials.line_mat.clone(),
        meshes,
        Vec3::new(0.0, 0.0, 0.0),
        &StrokeOptions::default()
    )
}

fn spawn_line(
    mut commands: &mut Commands,
    materials: &Res<Materials>,
    meshes: &mut ResMut<Assets<Mesh>>, 
    a: [CoordNum; 2],
    b: [CoordNum; 2]
) {
    commands
    .spawn(line_sprite(
        &materials,
        meshes,
        a,
        b
    ))
    .with(DebugLine)
    ;
}

fn spawn_circle(
    mut commands: &mut Commands,
    material: Handle<ColorMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>, 
    pos: [CoordNum; 2],
    r: f32,
) {
    commands.spawn(
        primitive(
            material,
            meshes,
            ShapeType::Circle(r),
            TessellationMode::Fill(&FillOptions::default()),
            Vec3::new(pos[0] as f32, pos[1] as f32, 0.).into(),
        )
    )
    .with(DebugCircle)
    ;
}
*/