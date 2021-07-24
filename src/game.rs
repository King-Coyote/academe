use crate::{
    input::MouseState,
    ui::*,
    nav::NavAgent,
};
use bevy::prelude::*;

mod aspects;
pub use aspects::*;
mod commands;
pub use commands::*;
mod objects;
pub use objects::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GameCommandQueue(vec![]))
            .add_system(execute_game_commands.exclusive_system())
            .add_system(appearance_added.system())
            ;
    }
}

// UTILITY FNS

pub fn spawn_standard_boi(pos: &Vec2, cmds: &mut Commands, mouse: &MouseState) {
    let mut entity = cmds.spawn();
    let spawned = entity.id();
    entity
        .insert(Appearance {
            filename: "textures/sprites/circle_lmao.png".to_string(),
            ..Default::default()
        })
        .insert(Body {
            strength: 10,
            coordination: 10,
            endurance: 10,
        })
        .insert(Movement{level: 1})
        .insert(NavAgent{dest: Some(Vec2::ZERO)})
        .insert(Transform::from_translation(pos.extend(100.0)))
        .insert(ClickHandlers {
            right: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
                cmds.spawn().insert(ContextMenuSpawn {
                    pos: mouse.ui_pos,
                    items: vec![ContextMenuItem {
                        label: "Test".to_owned(),
                        handlers: Some(ClickHandlers {
                            left: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
                                cmds.entity(spawned).insert(Despawning);
                            })),
                            ..Default::default()
                        }),
                    }],
                });
            })),
            ..Default::default()
        });
}
