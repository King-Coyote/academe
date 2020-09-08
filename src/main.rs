use bevy::prelude::*;
use crate::editor::EditorPlugin;

mod ui;
mod scripting;
mod space;
mod editor;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(EditorPlugin)
        .run();
}