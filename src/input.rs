use bevy::{input::{self, ElementState, mouse::{MouseButtonInput, MouseMotion, MouseWheel}}, prelude::*, render::camera::{CameraProjection, OrthographicProjection}};

pub struct MainCamera;

pub struct InputPlugin;

pub struct MouseState {
    pub screen_pos: Vec2,
    pub world_pos: Vec2,
    pub projected_pos: Vec2,
}

fn setup(mut commands: Commands) {
    commands.spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    
    commands.insert_resource(MouseState {
        screen_pos: Vec2::ZERO,
        world_pos: Vec2::ZERO,
        projected_pos: Vec2::ZERO,
    });
}

fn mouse_state(
    // events to get cursor position
    mut evr_cursor: EventReader<CursorMoved>,
    // need to get window dimensions
    windows: Res<Windows>,
    mut mouse_state: ResMut<MouseState>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>
) {
    let camera_transform = q_camera.iter().next().unwrap();
    for e in evr_cursor.iter() {
        // apply the camera transform
        let world_pos = get_world_pos(&e.position, camera_transform, &windows.get(e.id).unwrap());
        mouse_state.screen_pos = e.position;
        mouse_state.world_pos = Vec2::new(world_pos.x, world_pos.y);
        mouse_state.projected_pos = Vec2::new(world_pos.x, world_pos.y * 2.0);
    }
}

// fn click_world_system(
//     mut er_mouse: EventReader<MouseButtonInput>,
//     mut ew_worldmouse: EventWriter<MouseButtonInputWorld>,
//     windows: Res<Windows>,
//     q_camera: Query<&Transform, With<MainCamera>>
// ) {
//     let camera_transform = q_camera.iter().next().unwrap();
//     for e in er_mouse.iter() {
//         let world_pos = get_world_pos(&e.position, &camera_transform, &windows.get_primary());
//         ew_worldmouse.send(MouseButtonInputWorld {

//         })
//     }
// }

fn camera_control_system(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
) {
    let pan_button = MouseButton::Left;

    let (mut cam_position, mut projection) = query.iter_mut().next().unwrap();

    if input_mouse.pressed(pan_button) {
        for e in ev_motion.iter() {
            let window = windows.get_primary().unwrap();
            cam_position.translation.x -= e.delta.x;
            cam_position.translation.y += e.delta.y;
            projection.update(window.width() as f32, window.height() as f32);
        }
    }

}

// fn zoom_system(
//     mut whl: EventReader<MouseWheel>,
//     mut cam: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
//     windows: Res<Windows>,
// ) {
//     let delta_zoom: f32 = whl.iter().map(|e| e.y).sum();
//     if delta_zoom == 0. {
//         return;
//     }

//     let (mut pos, mut cam) = cam.single_mut().unwrap();

//     let window = windows.get_primary().unwrap();
//     let window_size = Vec2::new(window.width(), window.height());
//     let mouse_normalized_screen_pos =
//         (window.cursor_position().unwrap() / window_size) * 2. - Vec2::ONE;
//     let mouse_world_pos = pos.translation.truncate()
//         + mouse_normalized_screen_pos * Vec2::new(cam.right, cam.top) * cam.scale;

//     cam.scale -= ZOOM_SPEED * delta_zoom * cam.scale;
//     cam.scale = cam.scale.clamp(MIN_ZOOM, MAX_ZOOM);

//     pos.translation = (mouse_world_pos
//         - mouse_normalized_screen_pos * Vec2::new(cam.right, cam.top) * cam.scale)
//         .extend(pos.translation.z);
// }

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .add_system(mouse_state.system())
            // .add_system(click_world_system.system())
            .add_system(camera_control_system.system())
        ;
    }
}

// UTILITY FNS

fn get_world_pos(screen_pos: &Vec2, cam_transform: &Transform, window: &Window) -> Vec4 {
    let size = Vec2::new(window.width() as f32, window.height() as f32);
    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let p = *screen_pos - size / 2.0;
    cam_transform.compute_matrix() * p.extend(0.0).extend(1.0)
}