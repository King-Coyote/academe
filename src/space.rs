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
    utils::geometry::{point_inside_polygon,},
    ui::*,
    game::*,
};

// curently for rendering spaces and allowing them to be interacted with.
pub struct SpacePlugin;

fn setup(
    mut commands: Commands,
) {
    let points = vec![
        Vec2::new(0.0, 150.0),
        Vec2::new(300.0, 0.0),
        Vec2::new(0.0, -150.0),
        Vec2::new(-300.0, 0.0),
    ];

    let shape = shapes::Polygon {
        points: points.clone(),
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
        .insert(InteractablePolygon{points})
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
                        command: GameCommandType::Create("Mind".to_string()),
                        level: 5,
                    },
                    {
                        target: Target::LastCreated,
                        command: GameCommandType::Modify{
                            name: "Body".to_string(),
                            values: {
                                let mut ds = DynamicStruct::default();
                                ds.insert("strength", 20u32);
                                ds.insert("coordination", 20u32);
                                ds.insert("endurance", 20u32);
                                Arc::new(ds)
                            }
                        },
                        level: 4,
                    }
                ),
                closing: true
            },
            {
                label: "test",
                commands: game_commands!(),
                closing: true
            }
        ))
        .insert(InteractState(InteractStateEnum::Enabled))
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