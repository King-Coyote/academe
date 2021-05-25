use bevy::prelude::*;

mod aspects;
pub use aspects::*;
mod commands;
pub use commands::*;

pub struct GamePlugin;

fn test_command_aspects(
    query: Query<(&Body, &Mind)>,
) {
    for (body, mind) in query.iter() {
        println!("Mind and body found as {:?},\n {:?}", mind, body);
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .register_type::<Body>()
            .register_type::<Mind>()
            .register_type::<Spirit>()
            .register_type::<Appearance>()
            .insert_resource(GameCommandQueue(vec![]))
            .add_system(execute_game_commands.exclusive_system())
            .add_system(appearance_added.system())
            .add_system(test_command_aspects.system())
        ;
    }
}