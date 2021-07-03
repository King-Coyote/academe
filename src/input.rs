use bevy::{
    prelude::*,
    render::camera::{OrthographicProjection},
};

pub struct MainCamera;

pub struct InputPlugin;

#[derive(Clone, Default)]
pub struct MouseState {
    pub screen_pos: Vec2,
    pub ui_pos: Vec2,
    pub world_pos: Vec2,
    pub projected_pos: Vec2,
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    commands.insert_resource(MouseState {
        screen_pos: Vec2::ZERO,
        ui_pos: Vec2::ZERO,
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
    q_camera: Query<&Transform, With<MainCamera>>,
) {
    let camera_transform = q_camera.iter().next().unwrap();
    for e in evr_cursor.iter() {
        // apply the camera transform
        let window = windows.get_primary().unwrap();
        let world_pos = get_world_pos(&e.position, camera_transform, &window);
        mouse_state.screen_pos = e.position;
        mouse_state.ui_pos = Vec2::new(e.position.x, window.height() as f32 - e.position.y);
        mouse_state.world_pos = Vec2::new(world_pos.x, world_pos.y);
        mouse_state.projected_pos = Vec2::new(world_pos.x, world_pos.y * 2.0);
    }
}

fn camera_control_system(
    windows: Res<Windows>,
    mouse: Res<MouseState>,
    input_mouse: Res<Input<MouseButton>>,
    input_keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
) {
    let (mut cam_position, _) = query.iter_mut().next().unwrap();
    let window = windows.get_primary().unwrap();
    let pan_speed = 7.0;

    let mut movement = Vec2::ZERO;
    if input_keys.pressed(KeyCode::LControl)
        || input_keys.pressed(KeyCode::LShift)
        || input_keys.pressed(KeyCode::LAlt)
    {
        return;
    }
    if input_keys.pressed(KeyCode::W) {
        movement.y += pan_speed;
    }
    if input_keys.pressed(KeyCode::A) {
        movement.x -= pan_speed;
    }
    if input_keys.pressed(KeyCode::S) {
        movement.y -= pan_speed;
    }
    if input_keys.pressed(KeyCode::D) {
        movement.x += pan_speed;
    }

    cam_position.translation += movement.extend(0.0);
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(mouse_state.system())
            // .add_system(click_world_system.system())
            .add_system(camera_control_system.system());
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
