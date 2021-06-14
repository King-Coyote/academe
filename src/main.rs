#![allow(clippy::type_complexity, clippy::clippy::too_many_arguments, unused_variables, dead_code)]
#![feature(btree_retain, exact_size_is_empty)]

use bevy::prelude::*;
use input::*;
use space::*;
use ui::*;
use game::*;

#[macro_use]
mod macros;
mod input;
mod space;
mod utils;
mod ui;
mod game;

// dfault stages look like this:
// self.add_startup_stage(startup_stage::STARTUP)
// .add_startup_stage(startup_stage::POST_STARTUP)
// .add_stage(stage::FIRST)
// .add_stage(stage::EVENT_UPDATE)
// .add_stage(stage::PRE_UPDATE)
// .add_stage(stage::UPDATE)
// .add_stage(stage::POST_UPDATE)
// .add_stage(stage::LAST)

fn area_texture_test(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<AssetServer>,
) {
    let tex_handle: Handle<Texture> = assets.load("textures/render.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(tex_handle.into()),
        ..Default::default()
        // pub sprite: Sprite,
        // pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
        // pub material: Handle<ColorMaterial>,
        // pub main_pass: MainPass,
        // pub draw: Draw,
        // pub visible: Visible,
        // pub render_pipelines: RenderPipelines,
        // pub transform: Transform,
        // pub global_transform: GlobalTransform,
    });
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputPlugin)
        .add_plugin(SpacePlugin)
        .add_plugin(UiPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(area_texture_test.system())
        .run();
}