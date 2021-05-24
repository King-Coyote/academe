#![allow(clippy::type_complexity, clippy::clippy::too_many_arguments, unused_variables)]

use bevy::prelude::*;
use input::*;
use space::*;
use ui::*;
use game::*;

#[macro_use]
mod macros;
mod input;
mod space;
mod utils;
mod ui;
mod game;

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
        .add_plugin(UiPlugin)
        .add_plugin(GamePlugin)
        .run();
}