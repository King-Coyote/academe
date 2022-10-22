#![allow(
    clippy::type_complexity,
    clippy::clippy::too_many_arguments,
    unused_variables,
    dead_code
)]
#![feature(exact_size_is_empty)]

use bevy::{
    asset::AssetServerSettings,
    prelude::*
};
use game::*;
use input::*;
use space::*;
use ui::*;
use nav::*;
use debug::*;
use ai::*;
use bevy_htn::prelude::*;

#[macro_use]
mod macros;
mod game;
mod input;
mod space;
mod ui;
mod utils;
mod nav;
mod debug;
mod ai;

fn area_texture_test(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<AssetServer>,
) {
    // let tex_handle: Handle<Texture> = assets.load("textures/outdoors.png");
    // commands.spawn_bundle(SpriteBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     material: materials.add(tex_handle.into()),
    //     ..Default::default()
    // });
    spawn_standard_boi(Vec2::new(0.0, 0.0), &mut commands, false);
}

fn click_debug(
    mouse: Res<MouseState>,
    mouse_button: Res<Input<MouseButton>>,
) {
    if mouse_button.just_released(MouseButton::Left) {
        println!("Clicked at: {}", mouse.world_pos);
    }
}

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            asset_folder: "/home/alex/projects/bevyacad/assets".to_string(),
            ..default()
        })
        .add_startup_system(area_texture_test)
        .add_plugins(DefaultPlugins)
        .add_plugin(InputPlugin)
        .add_plugin(SpacePlugin)
        .add_plugin(UiPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(NavPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(AiPlugin)
        .add_system(click_debug)
        .run();
}
