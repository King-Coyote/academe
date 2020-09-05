use bevy::prelude::*;

pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(setup.system());
    }
}

fn setup() {

}