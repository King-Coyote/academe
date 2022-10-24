use crate::{
    input::MouseState,
    ui::*,
    nav::{NavAgent, NavAgentStrategy},
    ai::{ActorStore, EnemyContext},
};
use bevy::{prelude::*, text::Text2dSize};
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
    mouse: &MouseState,
    text_style: TextStyle,
    cmds: &mut Commands,
    is_enemy: bool,
) {
    info!("Spawning student at {}", pos);
    let mut entity = cmds.spawn();
    let spawned = entity.id();
    let filename = {
        if is_enemy {
            "textures/sprites/enemy.png".to_string()
        } else {
            "textures/sprites/player.png".to_string()
        }
    };
    entity
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
        .insert(NavAgent::with_strategy(NavAgentStrategy::FreeRoam))
        .insert(Transform::from_translation(pos.extend(100.0)))
        .insert(ClickHandlers {
            right: context_menu_handler!(cmds, mouse, {
                label: "Annihilate",
                action: {
                    cmds.entity(spawned).despawn_recursive();
                }
            }),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    "Entity",
                    text_style
                ),
                text_2d_size: Text2dSize{size: Vec2::new(100.0, 20.0)},
                transform: Transform::from_xyz(-25.0, 60.0, 0.0),
                ..default()
            });
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
    }
}
