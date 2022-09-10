use crate::{
    input::MouseState,
    ui::*,
    nav::NavAgent,
    ai::{ActorStore, EnemyContext},
};
use bevy::prelude::*;
use bevy_htn::prelude::*;

mod aspects;
pub use aspects::*;
mod area;
pub use area::*;

#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct Player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(appearance_added)
            ;
    }
}

// UTILITY FNS
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
                            // left: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
                            //     cmds.entity(spawned).insert(Despawning);
                            // })),
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
                actor_store: ActorStore {
                    move_target: None,
                    current_pos: pos,
                    wants_new_location: true,
                    current_time: 0.0,
                }
            })
            // TODO UPDATE
            // .insert(Planner::<EnemyContext>::default())
            ;
    } else {
        entity.insert(Player);
    }
}
