use crate::{
    game::*,
    input::*,
    ui::*,
    utils::geometry::*,
    nav::*,
};
use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::*,
    entity::ShapeBundle,
};

// curently for rendering spaces and allowing them to be interacted with.
pub struct SpacePlugin;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // TODO UPDATE
    //outdoors
    // let points = vec![
    //     Vec2::new(-884.20544, -75.55908),
    //     Vec2::new(-752.34406, -10.681641),
    //     Vec2::new(-389.2116, -201.02673),
    //     Vec2::new(-173.82013, -99.71777),
    //     Vec2::new(-173.34747, 40.61139),
    //     Vec2::new(-394.4218, 159.14343),
    //     Vec2::new(27.454956, 356.94482),
    //     Vec2::new(886.8773, -99.542725),
    //     Vec2::new(-24.293274, -530.56366),
    // ];
    // // //indoors
    // // let points = vec![
    // //     Vec2::new(-288.22113, -1.0939026),
    // //     Vec2::new(-72.20764, 99.17911),
    // //     Vec2::new(293.77295, -94.57794),
    // //     Vec2::new(77.69818, -196.9029),
    // // ];
    // let closure_points = points.clone();
    // let shape = shapes::Polygon {
    //     points: points.clone(),
    //     closed: true,
    // };
    // let bg_color = materials.add(Color::BLACK.into());
    // let navmesh = NavMesh::new(points.clone(), vec![]).unwrap();

    // // SPAWN THE BIG OL NAVMESH
    // commands
    //     .spawn()
    //     .insert(navmesh)
    //     .insert(Polygon::new(points))
    //     .insert(ObjectInteraction::default())
    //     .insert(Transform::from_xyz(0.0, 0.0, 10.0))
    //     .insert(ClickHandlers {
    //         right: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
    //             let world_pos = mouse.world_pos;
    //             let ui_pos = mouse.ui_pos;
    //             cmds.spawn().insert(ContextMenuSpawn {
    //                 pos: ui_pos,
    //                 items: vec![
    //                     ContextMenuItem {
    //                         label: "Spawn Enemy".to_string(),
    //                         handlers: Some(ClickHandlers {
    //                             left: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
    //                                 spawn_standard_boi(world_pos, cmds, true);
    //                             })),
    //                             ..Default::default()
    //                         }),
    //                     },
    //                 ],
    //             });
    //         })),
    //         ..Default::default()
    //     });

    // // SPAWN THE DOOR's POLYGON
    // let door_points = vec![
    //     Vec2::new(-301.0, -105.0),
    //     Vec2::new(-263.55817, -85.88843),
    //     Vec2::new(-262.0, -142.07367),
    //     Vec2::new(-300.70947, -159.18121),
    // ];
    // let mut highlighted = GeometryBuilder::build_as(
    //     &shapes::Polygon {
    //         points: door_points.clone(),
    //         closed: true,
    //     },
    //     DrawMode::Outlined {
    //         fill_mode: FillMode {
    //             color: Color::rgba(0.8, 0.1, 0.9, 0.25),
    //             ..Default::default()
    //         },
    //         outline_mode: StrokeMode {
    //             options: StrokeOptions {
    //                 line_width: 1.0,
    //                 ..Default::default()
    //             },
    //             color: Color::BLUE
    //         },
    //     },
    //     Transform::from_xyz(0.0, 0.0, 1000.0),
    // );
    // commands.spawn()
    //     .insert(Polygon::new(door_points))
    //     .insert(ObjectInteraction::default())
    //     .insert(Transform::from_xyz(0.0, 0.0, 11.0))
    //     .insert(Highlighted(false))
    //     .with_children(|parent| {
    //         parent.spawn_bundle(highlighted);
    //     })
    //     .insert(ClickHandlers {
    //         left: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
    //             info!("Clicked the door polygon!");
    //         })),
    //         ..Default::default()
    //     })
    //     ;
}

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShapePlugin)
            .add_startup_system(setup)
            ;
    }
}
