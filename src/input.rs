use bevy::{prelude::*};

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

fn world_coords_system(
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

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .add_system(world_coords_system.system())
        ;
    }
}