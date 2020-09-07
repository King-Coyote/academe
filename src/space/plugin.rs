use bevy::prelude::*;

pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(setup.system());
    }
}

// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let texture_handle = asset_server.load("assets/textures/tilesets/ruin_tiles_128x64.png").unwrap();
//     commands
//         .spawn(Camera2dComponents::default())
//         .spawn(SpriteComponents {
//             material: materials.add(texture_handle.into()),
//             // sprite: Sprite {
//             //     size: Vec2::new(120.0, 30.0),
//             // },
//             ..Default::default()
//         });
// }

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
        .spawn(Camera2dComponents::default());
    hardcoded_terraingen(64.0, 128.0, &mut commands, texture_atlas_handle);
}

// TODO maybe make this inline
// the x and y are in tile coords, not pixels
// fn map_to_screen(x: f32, y: f32, width: f32, height: f32) -> (f32, f32) {
//     // let x = x as f32;
//     // let y = y as f32;
//     // (
//     //     (x - y) * (width/2.0),
//     //     (x + y) * (height/2.0)
//     // )
// }

fn hardcoded_terraingen(
    width: f32, 
    height: f32,
    mut commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
) {
    let origin = (0.0, 0.0);
    let indices: Vec<Vec<u32>> = vec![
        vec![0,     0,      0,      0],
        vec![0,     0,      0,      0],
        vec![0,     0,      0,      0],
        vec![0,     0,      0,      0],
    ];
    for (j, row) in indices.iter().enumerate() {
        for (i, tile) in row.iter().enumerate() {
            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(*tile),
                    translation: Translation::new((i as f32)*width*2.0, (j as f32) * height, 0.0),
                    ..Default::default()
                });
        }
    }

}