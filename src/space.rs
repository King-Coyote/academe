use crate::{
    game::*,
    input::*,
    ui::*,
    utils::geometry::*,
    nav::*,
};
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
    
    let hole = vec![
        Vec2::new(-500.0, -90.0),
        Vec2::new(-300.0, -90.0),
        Vec2::new(-300.0, -60.0),
        Vec2::new(-500.0, -60.0),
    ];
    let navmesh = {
        let mut builder = NavMeshBuilder::new();
        builder
            .with_boundary(&*points)
            .with_hole(&hole);
        builder.build().unwrap()
    };

    commands
        .spawn()
        .insert(navmesh)
        .insert(Polygon { points, max_dim })
        .insert(ObjectInteraction::default())
        .insert(Transform::from_xyz(0.0, 0.0, 10.0))
        .insert(ClickHandlers {
            right: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
                let world_pos = mouse.world_pos;
                let ui_pos = mouse.ui_pos;
                cmds.spawn().insert(ContextMenuSpawn {
                    pos: ui_pos,
                    items: vec![
                        ContextMenuItem {
                            label: "Spawn Enemy".to_string(),
                            handlers: Some(ClickHandlers {
                                left: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
                                    spawn_standard_boi(world_pos, cmds, true);
                                })),
                                ..Default::default()
                            }),
                        },
                    ],
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
