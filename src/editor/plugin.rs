use bevy::prelude::*;
use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    window::CursorMoved,
};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .init_resource::<MouseState>()
        .add_startup_system(setup.system())
        .add_system(tile_placement_system.system());
    }
}

#[derive(Default)]
struct MouseState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

struct PlacingTile;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load_sync(
            &mut textures,
            "assets/textures/tilesets/ruin_floor_tiles_64x32.png",
        )
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 16, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(
            SpriteSheetComponents {
                texture_atlas: texture_atlas_handle.clone(),
                translation: Translation::new(0.0, 0.0, 0.0),
                sprite: TextureAtlasSprite::new(0),
                ..Default::default()
            },
        )
        .with(PlacingTile);
}

fn tile_placement_system(
    mut commands: Commands,
    mut state: ResMut<MouseState>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    _tile: &PlacingTile,
    mut translation: Mut<Translation>,
    // sprite: &TextureAtlasSprite,
    // mut tile_query: Query<(&PlacingTile, &mut Translation)>,
) {
    // for event in state.mouse_button_event_reader.iter(&mouse_button_input_events)
    // {
    //     println!("{:?}", event);
    // }

    // for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
    //     println!("{:?}", event);
    // }

    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        translation.set_x(event.position.x());
        translation.set_y(event.position.y());
    }

    // for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
    //     println!("{:?}", event);
    // }
}