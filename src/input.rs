use bevy::{
    prelude::*,
};
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

#[derive(Component)]
pub struct MainCamera;

#[derive(Clone, Default)]
pub struct MouseState {
    pub screen_pos: Vec2,
    pub ui_pos: Vec2,
    pub world_pos: Vec2,
    pub projected_pos: Vec2,
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq)]
pub enum CameraMovement {
    PanUp,
    PanDown,
    PanLeft,
    PanRight,
    Zoom,
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(Camera2dBundle::default())
        .insert_bundle(InputManagerBundle::<CameraMovement> {
            input_map: InputMap::default()
                .insert(KeyCode::W, CameraMovement::PanUp)
                .insert(KeyCode::S, CameraMovement::PanDown)
                .insert(KeyCode::A, CameraMovement::PanLeft)
                .insert(KeyCode::D, CameraMovement::PanRight)
                .insert(SingleAxis::mouse_wheel_y(), CameraMovement::Zoom)
                .build(),
            action_state: ActionState::default(),
        })
        .insert(MainCamera);
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
        let world_pos = get_world_pos(&e.position, camera_transform, window);
        mouse_state.screen_pos = e.position;
        mouse_state.ui_pos = Vec2::new(e.position.x, window.height() as f32 - e.position.y);
        mouse_state.world_pos = Vec2::new(world_pos.x, world_pos.y);
        mouse_state.projected_pos = Vec2::new(world_pos.x, world_pos.y * 2.0);
    }
}

const CAMERA_ZOOM_RATE: f32 = 0.05;
const CAMERA_SPEED: f32 = 7.0;

fn camera_control_system(
    mut query: Query<(&mut Transform, &ActionState<CameraMovement>, &mut OrthographicProjection), With<Camera2d>>,
) {
    let (mut cam_transform, action_state, mut projection) = query.single_mut();
    let pan_speed = 7.0;
    let zoom_delta = action_state.value(CameraMovement::Zoom);

    let mut movement = Vec2::ZERO;
    if action_state.pressed(CameraMovement::PanUp) {
        movement.y += CAMERA_SPEED;
    }
    if action_state.pressed(CameraMovement::PanLeft) {
        movement.x -= CAMERA_SPEED;
    }
    if action_state.pressed(CameraMovement::PanDown) {
        movement.y -= CAMERA_SPEED;
    }
    if action_state.pressed(CameraMovement::PanRight) {
        movement.x += CAMERA_SPEED;
    }
    projection.scale *= 1. - zoom_delta * CAMERA_ZOOM_RATE * -1.0;

    cam_transform.translation += movement.extend(0.0);
}

impl Plugin for InputPlugin {
    // this is where we set up our plugin
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(InputManagerPlugin::<CameraMovement>::default())
            .init_resource::<MouseState>()
            .add_system(mouse_state)
            // .add_system(click_world_system)
            .add_system(camera_control_system)
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
