use bevy::{
    core::FixedTimestep,
    prelude::*,
};

mod nav_agent;
pub use nav_agent::*;
mod navmesh;
pub use navmesh::*;
mod graph;

const TIME_STEP: f32 = 1.0 / 60.0;

pub struct NavPlugin;

impl Plugin for NavPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(navagent_system.system())
        )
        ;
    }
}
