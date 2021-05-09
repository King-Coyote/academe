use bevy::{input::mouse, prelude::*};

pub struct MainCamera;

pub struct MainCameraPlugin;

struct MouseState {
    screen_pos: Vec2,
    world_pos: Vec2,
}

fn setup(mut commands: Commands) {
    commands.spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    
    commands.insert_resource(MouseState {
        screen_pos: Vec2::new(0.0, 0.0),
        world_pos: Vec2::new(0.0, 0.0),
    });
}

fn world_coords_system(
    // events to get cursor position
    mut evr_cursor: EventReader<CursorMoved>,
    // need to get window dimensions
    wnds: Res<Windows>,
    mut mouse_state: ResMut<MouseState>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>
) {
    let camera_transform = q_camera.iter().next().unwrap();

    for ev in evr_cursor.iter() {
        // get the size of the window that the event is for
        let wnd = wnds.get(ev.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = ev.position - size / 2.0;
        // apply the camera transform
        let world_pos = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        mouse_state.screen_pos = ev.position;
        mouse_state.world_pos = Vec2::new(world_pos.x, world_pos.y);
    }
}

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .add_system(world_coords_system.system())
        ;
    }
}

// fn startup(commands: &mut Commands) {
//     let e = commands
//         .spawn(Camera2dBundle::default())
//         .with(MainCamera)
//         .current_entity()
//         .unwrap();

//     commands.insert_resource(MouseState {
//         main_cam: e,
//         cursor_moved_event_reader: Default::default(),
//         // mouse_wheel_event_reader: EventReader<MouseWheel>,
//         screen_pos: Vec2::new(0.0, 0.0),
//         world_pos: Vec2::new(0.0, 0.0),
//     });
// }

// fn world_coords_system(
//     windows: Res<Windows>,
//     cursor_moved_events: Res<Events<CursorMoved>>,
//     mut mouse_state: ResMut<MouseState>,
//     mut query_cam: Query<(&MainCamera, &mut Transform)>,
// ) {
//     let window = windows.get_primary().unwrap();
//     let camera_transform = query_cam.get_mut(mouse_state.main_cam).unwrap().1;

//     for event in mouse_state.cursor_moved_event_reader.iter(&cursor_moved_events) {
//         let size = Vec2::new(
//             window.width() as f32, 
//             window.height() as f32
//         );
//         let projection = event.position - size / 2.0;
//         let world_coords = camera_transform.compute_matrix() * projection.extend(0.0).extend(1.0);

//         mouse_state.screen_pos = Vec2::new(
//             event.position.x,
//             event.position.y
//         );
//         mouse_state.world_pos = Vec2::new(
//             world_coords.x,
//             world_coords.y,
//         );
//     }
// }