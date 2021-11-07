use crate::{
    game::*,
    input::MouseState,
};
use bevy::{
    prelude::*,
};

mod interaction;
pub use interaction::*;
mod elements;
pub use elements::*;
mod editor_interface;
pub use editor_interface::*;
mod debug;
pub use debug::*;

pub struct UiPlugin;

fn setup(mut commands: Commands, materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(UiCameraBundle::default());
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .register_type::<Polygon>()
            .init_resource::<MainStyle>()
            .insert_resource(InteractableOrder::default())
            .add_startup_system(setup.system())
            .add_system_set(SystemSet::new()
                .label("interaction")
                .with_system(polygon_interact_system.system())
                .with_system(interaction_with_handlers.system())
                .with_system(capture_interactions.system())
                .with_system(object_interaction_ordering.system())
                .with_system(object_interaction_handling.system())
                .with_system(highlight_system.system())
                .with_system(appearance_interact_system.system())
            )
            .add_system_set(SystemSet::new()
                .label("elements")
                .with_system(button.system())
                .with_system(popup_system.system())
                .with_system(context_menu_spawn.system())
            )
            .add_system_set(SystemSet::new()
                .label("editor_interface")
                .with_system(spawn_debug_ui.system())
                .with_system(save_load.exclusive_system())
            )
            ;
    }
}
