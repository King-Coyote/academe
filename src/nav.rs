use bevy::{
    core::FixedTimestep,
    prelude::*,
    input::{mouse::MouseButtonInput, ElementState,},
};
use crate::input::*;

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
    q_navmesh: Query<&NavMesh>,
) {
    for e in er_mouse.iter() {
        if e.state != ElementState::Released {
            continue;
        }
        let point = mouse.world_pos;
        info!("Clicked at: {}", point);
        for n in q_navmesh.iter() {
            if n.points_have_los(&point, &Vec2::new(140.0, -184.0)) {
                info!("LOS to 0,0");
            }
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
