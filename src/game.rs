use std::default;

use crate::{
    input::MouseState,
    ui::*,
    nav::NavAgent,
    ai_demo::EnemyContext,
};
use bevy::prelude::*;
use bevy_htn::prelude::*;

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

pub struct Enemy;
pub struct Player;

pub fn spawn_standard_boi(
    pos: Vec2,
    cmds: &mut Commands,
    is_enemy: bool,
) {
    let mut entity = cmds.spawn();
    let spawned = entity.id();
    let filename = {
        if is_enemy {
            "textures/sprites/enemy.png".to_string()
        } else {
            "textures/sprites/player.png".to_string()
        }
    };
    let entity = entity
        .insert(Appearance {
            filename,
            ..Default::default()
        })
        .insert(Body {
            strength: 10,
            coordination: 10,
            endurance: 10,
        })
        .insert(Movement{level: 1})
        .insert(Enemy)
        .insert(NavAgent::default())
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
    if is_enemy {
        entity.insert(Enemy)
            .insert(EnemyContext {
                name: "BeEnemy".to_string(),
                state: ContextState::default(),
                move_target: None,
                current_pos: pos,
                wants_new_location: true,
                current_time: 0.0,
            })
            .insert(Planner::<EnemyContext>::default())
            ;
    } else {
        entity.insert(Player);
    }
}
