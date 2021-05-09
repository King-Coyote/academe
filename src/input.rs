use bevy::{
    prelude::*,
    input::{
        ElementState,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    }
};

pub struct MainCamera;

pub struct InputPlugin;

pub struct MouseState {
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
        // get the size of the window that the event is for
        let window = windows.get(e.id).unwrap();
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = e.position - size / 2.0;
        // apply the camera transform
        let world_pos = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        mouse_state.screen_pos = e.position;
        mouse_state.world_pos = Vec2::new(world_pos.x, world_pos.y);
    }
}

fn click_system(
    mouse: Res<MouseState>,
    mut er_mouse: EventReader<MouseButtonInput>,
) {
    for e in er_mouse.iter() {
        if e.button == MouseButton::Right && e.state == ElementState::Released {
            println!("Ya done click't at screen: {}, world: {}", mouse.screen_pos, mouse.world_pos);
        }
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .add_system(mouse_state.system())
            .add_system(click_system.system())
        ;
    }
}