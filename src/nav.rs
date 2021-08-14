use bevy::{
    core::FixedTimestep,
    prelude::*,
    input::{mouse::MouseButtonInput, ElementState,},
};
use crate::{
    input::*,
    debug::*,
};

mod nav_agent;
pub use nav_agent::*;
mod navmesh;
pub use navmesh::*;
mod graph;

const TIME_STEP: f32 = 1.0 / 60.0;

pub struct NavPlugin;

fn click_pathfind_debug_system(
    mut commands: Commands,
    mouse: Res<MouseState>,
    mut er_mouse: EventReader<MouseButtonInput>,
    q_navmesh: Query<(Entity, &NavMesh)>,
    mut q_navagent: Query<(Entity, &mut NavAgent,)>,
) {
    for e in er_mouse.iter() {
        if e.state != ElementState::Released || e.button != MouseButton::Left {
            continue;
        }
        let point = mouse.world_pos;
        for (entity, navmesh) in q_navmesh.iter() {
            commands.entity(entity).with_children(|parent| {
                let path = navmesh.find_path(&point, &Vec2::new(330.0, -140.0)).unwrap();
                for node in path.iter() {
                    parent.spawn_bundle(vertex_bundle(*node, Color::ORANGE));
                }
                if let Ok((_, mut agent)) = q_navagent.single_mut() {
                    agent.path = path;
                }
            });
        }
    }
}

impl Plugin for NavPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(navagent_system.system())
        )
        .add_system(click_pathfind_debug_system.system())
        ;
    }
}
