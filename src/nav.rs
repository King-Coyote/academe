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
    q_navmesh: Query<()>,
) {
    for e in er_mouse.iter() {
        if e.state != ElementState::Released {
            continue;
        }
        let point = mouse.world_pos;
        info!("Clicked at: {}", point);
        
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
