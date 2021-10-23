use crate::{
    game::*,
    input::*,
    ui::*,
    utils::geometry::*,
    nav::*,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// curently for rendering spaces and allowing them to be interacted with.
pub struct SpacePlugin;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    //outdoors
    let points = vec![
        Vec2::new(-884.20544, -75.55908),
        Vec2::new(-752.34406, -10.681641),
        Vec2::new(-389.2116, -201.02673),
        Vec2::new(-173.82013, -99.71777),
        Vec2::new(-173.34747, 40.61139),
        Vec2::new(-394.4218, 159.14343),
        Vec2::new(27.454956, 356.94482),
        Vec2::new(886.8773, -99.542725),
        Vec2::new(-24.293274, -530.56366),
    ];
    // //indoors
    // let points = vec![
    //     Vec2::new(-288.22113, -1.0939026),
    //     Vec2::new(-72.20764, 99.17911),
    //     Vec2::new(293.77295, -94.57794),
    //     Vec2::new(77.69818, -196.9029),
    // ];
    let closure_points = points.clone();
    let max_dim = max_polygon_width(&points);
    let shape = shapes::Polygon {
        points: points.clone(),
        closed: true,
    };
    let bg_color = materials.add(Color::BLACK.into());
    let navmesh = NavMesh::new(points.clone(), vec![]).unwrap();
    // let navmesh = {
    //     let mut builder = NavMeshBuilder::new();
    //     builder
    //         .with_boundary(&*points);
    //         // .with_hole(&hole);
    //     builder.build().unwrap()
    // };

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
            .add_startup_system(setup.system())
            ;
    }
}
