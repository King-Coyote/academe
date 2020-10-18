use bevy::prelude::*;
use crate::editor::EditorPlugin;
use crate::ai::AiPlugin;

mod ui;
mod scripting;
mod space;
mod editor;
mod ai;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(AiPlugin)
        .run();
}