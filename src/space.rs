use std::{
    marker::PhantomData,
    sync::Arc,
};

use bevy::{
    input::{
        ElementState,
        mouse::{MouseButtonInput},
    },
    prelude::*,
    reflect::DynamicStruct,
};
use bevy_prototype_lyon::prelude::*;
use crate::{
    input::*,
    utils::geometry::*,
    ui::*,
    game::*,
};

// curently for rendering spaces and allowing them to be interacted with.
pub struct SpacePlugin;

fn setup(
    mut commands: Commands,
) {
    let points = Arc::new(vec![
        Vec2::new(0.0, 150.0),
        Vec2::new(300.0, 0.0),
        Vec2::new(0.0, -150.0),
        Vec2::new(-300.0, 0.0),
    ]);
    let closure_points = points.clone();
    let max_dim = max_polygon_width(&points);
    let shape = shapes::Polygon {
        points: (*points).clone(),
        closed: true
    };
    
    commands.spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(Color::TEAL, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(5.0),
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .insert(Polygon{points})
        .insert(Interactable {
            min_dist: max_dim / 2.0,
            mouse_inside: Some(Box::new(move |mouse: &MouseState| {
                point_inside_polygon(&mouse.world_pos, &*closure_points)
            })),
            ..Default::default()
        })
        .insert(context_menu!(
            {
                label: "Spawn creature",
                commands: game_commands!(
                    {
                        target: Target::World(None),
                        command: GameCommandType::Create("Body".to_string()),
                        level: 5,
                    },
                    {
                        target: Target::LastCreated,
                        command: GameCommandType::Create("Appearance".to_string()),
                        level: 5,
                    },
                    {
                        target: Target::LastCreated,
                        command: GameCommandType::Modify{
                            name: "Appearance".to_string(),
                            values: dynamic_struct!(
                                {"filename", "durr".to_string()}
                            )
                        },
                        level: 4,
                    }
                ),
                closing: true
            }
        ))
    ;
}

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(ShapePlugin)
            .add_startup_system(setup.system())
        ;
    }
}