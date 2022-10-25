use crate::{
    game::*,
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

fn setup(
    // mut commands: Commands, 
    materials: ResMut<Assets<ColorMaterial>>
) {
    // TODO UPDATE probably just delete this fam
    // commands.spawn_bundle(UiCameraBundle::default());
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Polygon>()
            .init_resource::<MainStyle>()
            .insert_resource(InteractableOrder::default())
            .add_startup_system(setup)
            .add_system_set(SystemSet::new()
                .label("interaction")
                .with_system(polygon_interact_system)
                .with_system(interaction_with_handlers)
                .with_system(capture_interactions)
                .with_system(object_interaction_ordering)
                .with_system(object_interaction_handling)
                // .with_system(highlight_system)
                .with_system(appearance_interact_system)
            )
            .add_system_set(SystemSet::new()
                .label("elements")
                .with_system(button)
                .with_system(popup_system)
                .with_system(context_menu_spawn)
            )
            .add_system_set(SystemSet::new()
                .label("editor_interface")
                .with_system(spawn_debug_ui)
                .with_system(save_load.exclusive_system())
            )
            ;
    }
}
