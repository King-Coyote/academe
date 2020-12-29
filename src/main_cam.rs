use bevy::prelude::*;

pub struct MainCamera;

pub struct MainCameraPlugin;

struct MouseState {
    // mouse_button_event_reader: EventReader<MouseButtonInput>,
    // mouse_motion_event_reader: EventReader<MouseMotion>,
    main_cam: Entity,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    // mouse_wheel_event_reader: EventReader<MouseWheel>,
    screen_pos: Vec2,
    world_pos: Vec2,
}

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(startup.system())
            .add_system(world_coords_system.system())
        ;
    }
}

fn startup(commands: &mut Commands) {
    let e = commands
        .spawn(Camera2dBundle::default())
        .with(MainCamera)
        .current_entity()
        .unwrap();

    commands.insert_resource(MouseState {
        main_cam: e,
        cursor_moved_event_reader: Default::default(),
        // mouse_wheel_event_reader: EventReader<MouseWheel>,
        screen_pos: Vec2::new(0.0, 0.0),
        world_pos: Vec2::new(0.0, 0.0),
    });
}

fn world_coords_system(
    windows: Res<Windows>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut mouse_state: ResMut<MouseState>,
    mut query_cam: Query<(&MainCamera, &mut Transform)>,
) {
    let window = windows.get_primary().unwrap();
    let camera_transform = query_cam.get_mut(mouse_state.main_cam).unwrap().1;

    for event in mouse_state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        let size = Vec2::new(
            window.width() as f32, 
            window.height() as f32
        );
        let projection = event.position - size / 2.0;
        let world_coords = camera_transform.compute_matrix() * projection.extend(0.0).extend(1.0);

        mouse_state.screen_pos = Vec2::new(
            event.position.x,
            event.position.y
        );
        mouse_state.world_pos = Vec2::new(
            world_coords.x,
            world_coords.y,
        );
    }
}