use bevy::prelude::*;
use input::*;
use space::*;

mod input;
mod space;

// dfault stages look like this:
// self.add_startup_stage(startup_stage::STARTUP)
// .add_startup_stage(startup_stage::POST_STARTUP)
// .add_stage(stage::FIRST)
// .add_stage(stage::EVENT_UPDATE)
// .add_stage(stage::PRE_UPDATE)
// .add_stage(stage::UPDATE)
// .add_stage(stage::POST_UPDATE)
// .add_stage(stage::LAST)

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputPlugin)
        .add_plugin(SpacePlugin)
        .run();
}