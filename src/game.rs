use bevy::prelude::*;

mod aspects;
pub use aspects::*;
mod area;
pub use area::*;
mod entities;
pub use entities::*;

#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct Player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(appearance_added)
            ;
    }
}