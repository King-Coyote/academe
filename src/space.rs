use std::{
    sync::Arc,
};
use bevy::{
    prelude::*,
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
    mut materials: ResMut<Assets<ColorMaterial>>,
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

    let bg_color = materials.add(Color::BLACK.into());
    
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
        .insert(InteractableObject {
            min_dist: max_dim / 2.0,
            mouse_inside: Some(Box::new(move |pos: &Vec2, mouse: &MouseState| {
                point_inside_polygon(&mouse.world_pos, &*closure_points)
            })),
            ..Default::default()
        })
        .insert(ClickHandlers {
            right: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
                let world_pos = mouse.world_pos;
                let ui_pos = mouse.ui_pos;
                cmds.spawn().insert(ContextMenuSpawn {
                    pos: ui_pos,
                    items: vec![
                        ContextMenuItem {
                            label: "Spawn test".to_string(),
                            handlers: Some(ClickHandlers {
                                left: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
                                    spawn_standard_boi(&world_pos, cmds, mouse);
                                })),
                                ..Default::default()
                            })
                        }
                    ]
                });
            })),
            ..Default::default()
        })
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