use bevy::prelude::*;
use crate::ui::UiPlugin;
use crate::scripting::ScriptingPlugin;
use crate::space::SpacePlugin;

mod ui;
mod scripting;
mod space;

fn main() {
    App::build()
        .add_default_plugins()
        // .add_plugin(UiPlugin)
        .add_plugin(ScriptingPlugin)
        .add_plugin(SpacePlugin)
        .run();
}