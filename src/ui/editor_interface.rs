use crate::ui::*;
use bevy::{reflect::TypeRegistryArc};
use std::{
    fs::File,
    io::prelude::*,
    path::Path,
};

const SAVE_PATH: &str = "/home/alex/projects/bevyacad/assets/scenes/editor_test.scn.ron";

pub fn save_load(
    world: &mut World,
    // keys: Res<Input<KeyCode>>,
    // scene_loader: Res<SceneLoader>,
) {
    let keys = world.get_resource::<Input<KeyCode>>().unwrap();
    let registry = world.get_resource::<TypeRegistryArc>().unwrap();
    // save
    if keys.pressed(KeyCode::LControl) && keys.just_released(KeyCode::S) {
        info!("Saving scene to {}", SAVE_PATH);
        let scene = DynamicScene::from_world(world, registry);
        let ron = scene.serialize_ron(registry).unwrap();
        info!("Serialized scene to {}", ron);
        write_scene(&SAVE_PATH, ron.as_bytes()).unwrap();
    }
    // load the scene ya filthy animal
    if keys.pressed(KeyCode::LControl) && keys.just_released(KeyCode::L) {
        info!("Loading scene from {}", SAVE_PATH);
        let scene_handle: Handle<DynamicScene> = {
            let asset_server = world.get_resource::<AssetServer>().unwrap();
            asset_server.load(SAVE_PATH)
        };
        // SceneSpawner can "spawn" scenes. "Spawning" a scene creates a new instance of the scene in
        // the World with new entity ids. This guarantees that it will not overwrite existing
        // entities
        let mut scene_spawner = world.get_resource_mut::<SceneSpawner>().unwrap();
        scene_spawner.spawn_dynamic(scene_handle);
        scene_spawner.is_changed();
    }
}

fn write_scene<P>(path: &P, data: &[u8]) -> std::io::Result<()>
where P: AsRef<Path> 
{
    info!("Writing to path {}", path.as_ref().display());
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

// pub fn polygon_build_controls(
//     mut commands: Commands,
//     mouse: Res<MouseState>,
//     mut poly_build: ResMut<PolygonBuilder>,
//     mut er_mouse: EventReader<MouseButtonInput>,
//     key_input: Res<Input<KeyCode>>,
// ) {
//     // if key_input.just_released(KeyCode::Escape) {
//     //     poly_build.building = false;
//     // }
//     // for e in er_mouse.iter() {
//     //     if e.button != MouseButton::Left
//     //     || e.state != ElementState::Released {
//     //         continue;
//     //     }

//     //     if key_input.pressed(KeyCode::LControl)
//     //     && !poly_build.building
//     //     {
//     //         poly_build.building = true;
//     //         poly_build.debug_ui = Some(commands.spawn().id());
//     //     }
//     //     if !poly_build.building {
//     //         continue;
//     //     }
//     //     let ui_entity = poly_build.debug_ui.unwrap();
//     //     if !poly_build.points.is_empty()
//     //     && mouse.world_pos.abs_diff_eq(*poly_build.points.get(0).unwrap(), 20.0) {
//     //         commands.entity(ui_entity).despawn();
//     //         poly_build.debug_ui = None;
//     //         // spawn a polygon here.
//     //     } else {
//     //         poly_build.points.push(mouse.world_pos);
//     //         let mut entity_build = commands.entity(ui_entity);
//     //         entity_build.insert(DebugCircleSpawn {
//     //             radius: 10.0,
//     //             center: mouse.world_pos,
//     //             color: Color::RED,
//     //         });
//     //         let num_points = poly_build.points.len();
//     //         if poly_build.points.len() > 1 {
//     //             entity_build.insert(DebugLineSpawn {
//     //                 origin: *poly_build.points.get(num_points - 1).unwrap(),
//     //                 dest: *poly_build.points.get(num_points).unwrap(),
//     //                 color: Color::RED,
//     //                 thickness: 2.0,
//     //             });
//     //         }
//     //     }
//     // }
// }
