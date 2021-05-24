use bevy::prelude::*;

mod aspects;
pub use aspects::*;
mod commands;
pub use commands::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .register_type::<Body>()
            .insert_resource(GameCommandQueue(vec![]))
            .add_system(execute_game_commands.exclusive_system())
        ;
    }
}