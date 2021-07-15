use crate::{game::*, input::*, ui::*, utils::geometry::*};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::sync::Arc;

// curently for rendering spaces and allowing them to be interacted with.
pub struct SpacePlugin;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let points = Arc::new(vec![
        Vec2::new(-711.90466, -97.48465),
        Vec2::new(-244.30817, 137.44873),
        Vec2::new(42.09662, -4.9219666),
        Vec2::new(331.9917, 133.8147),
        Vec2::new(528.577, 36.042236),
        Vec2::new(192.21948, -133.64969),
        Vec2::new(330.4182, -201.00244),
        Vec2::new(369.19006, -186.15796),
        Vec2::new(270.9057, -138.5155),
        Vec2::new(568.0725, 13.116638),
        Vec2::new(796.41724, -98.02606),
        Vec2::new(42.268982, -475.73297),
    ]);
    let closure_points = points.clone();
    let max_dim = max_polygon_width(&points);
    let shape = shapes::Polygon {
        points: (*points).clone(),
        closed: true,
    };

    let bg_color = materials.add(Color::BLACK.into());

    commands
        .spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(Color::rgba(0.0, 0.3, 0.75, 0.25), Color::BLUE),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .insert(Polygon { points, max_dim })
        .insert(ObjectInteraction::default())
        // .insert(InteractableObject {
        //     min_dist: max_dim / 2.0,
        //     mouse_inside: Some(Box::new(move |pos: &Vec2, mouse: &MouseState| {
        //         point_inside_polygon(&mouse.world_pos, &*closure_points)
        //     })),
        //     ..Default::default()
        // })
        .insert(ClickHandlers {
            right: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
                let world_pos = mouse.world_pos;
                let ui_pos = mouse.ui_pos;
                cmds.spawn().insert(ContextMenuSpawn {
                    pos: ui_pos,
                    items: vec![ContextMenuItem {
                        label: "Spawn test".to_string(),
                        handlers: Some(ClickHandlers {
                            left: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
                                spawn_standard_boi(&world_pos, cmds, mouse);
                            })),
                            ..Default::default()
                        }),
                    }],
                });
            })),
            ..Default::default()
        });
}

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(ShapePlugin)
            .add_startup_system(setup.system());
    }
}
