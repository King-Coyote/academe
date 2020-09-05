use bevy::prelude::*;
use crate::ui::UiPlugin;
use crate::scripting::ScriptingPlugin;

mod ui;
mod scripting;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(UiPlugin)
        .add_plugin(ScriptingPlugin)
        .run();
}