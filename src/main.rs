#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    unused_variables,
    dead_code
)]
#![feature(exact_size_is_empty)]

use std::collections::HashMap;
use bevy::{
    asset::AssetServerSettings,
    prelude::*, ecs::entity
};
use game::*;
use input::*;
use space::*;
use ui::*;
use nav::*;
use debug::*;
use ai::*;
use bevy_htn::prelude::*;
use bevy_prototype_lyon::prelude::*;

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

#[derive(Default)]
pub struct RoomsRegistry {
    pub map: HashMap<String, Entity>,
}

fn click_debug(
    mouse: Res<MouseState>,
    mouse_button: Res<Input<MouseButton>>,
) {
    if mouse_button.just_released(MouseButton::Left) {
        println!("Clicked at: {}", mouse.world_pos);
    }
}

fn create_hardcoded_rooms(
    mut commands: Commands,
    mut rooms_reg: ResMut<RoomsRegistry>,
    style: Res<MainStyle>
) {
    let rooms = vec![
        ("Dorms", Vec2::new(-100.0, 0.0)),
        ("Refactory", Vec2::new(100.0, 200.0)),
        ("Arboretum", Vec2::new(0.0, -400.0)),
        ("Study halls", Vec2::new(300.0, 0.0)),
    ];
    for room in rooms.iter() {
        let entity = spawn_room(&mut commands, room.0, room.1, &style.text);
        rooms_reg.map.insert(room.0.to_string(), entity);
    }
}

fn right_click_nothing(
    mut commands: Commands,
    mouse: Res<MouseState>,
    style: Res<MainStyle>,
    mouse_button: Res<Input<MouseButton>>,
    order: Res<InteractableOrder>,
) {
    let right_clicked_on_nothing = mouse_button.just_released(MouseButton::Right)
        && order.ui_blocking.is_none()
        && order.current.is_none();
    if right_clicked_on_nothing {
        let text_style = style.text.clone();
        commands.spawn()
            .insert(context_menu!(commands, mouse, 
                {
                    label: "Spawn student",
                    action: {
                        spawn_standard_boi(mouse.world_pos, mouse, text_style.clone(), commands, false);
                    }
                }
            ));
    }
}

fn spawn_test_rhombus(
    mut commands: Commands,
    mouse: Res<MouseState>,
) {
    let points = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(100.0, 0.0),
        Vec2::new(75.0, 100.0),
        Vec2::new(-25.0, 100.0),
    ];
    let polygon_shape = GeometryBuilder::build_as(
        &shapes::Polygon {
            points: points.clone(),
            closed: true,
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLUE, 2.0),
        },
        Transform::default(),
    );
    let mut entity_builder = commands.spawn();
    let entity = entity_builder.id();
    entity_builder
        .insert(Polygon::new(points))
        .insert(Transform::from_xyz(0.0, 0.0, 10.0))
        .insert(ObjectInteraction::default())
        .insert(ClickHandlers {
            left: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
                info!("Clicked the polygon!");
            })),
            right: context_menu_handler!(commands, mouse,
                {
                    label: "Delete",
                    action: {
                        commands.entity(entity).despawn_recursive();
                    }
                },
                {
                    label: "Hello foe",
                    action: {
                        info!("Clicked second item in CM");
                    }
                }
            ),
            ..Default::default()
        })
        .insert_bundle(polygon_shape)
        ;
}

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            asset_folder: "/home/alex/projects/academe/assets".to_string(),
            ..default()
        })
        // .add_startup_system(area_texture_test)
        .init_resource::<RoomsRegistry>()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputPlugin)
        .add_plugin(SpacePlugin)
        .add_plugin(UiPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(NavPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(AiPlugin)
        // .add_system(click_debug)
        // .add_startup_system(spawn_test_rhombus)
        .add_startup_system(create_hardcoded_rooms)
        .add_system(right_click_nothing)
        .run();
}
