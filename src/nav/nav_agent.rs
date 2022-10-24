use bevy::{
    prelude::*,
    input::{
        // ElementState,
        mouse::{MouseButtonInput, MouseButton},
    },
};
use crate::{
    game::*,
    input::*,
    nav::*,
};

const SPEED_MULT: f32 = 5.0;

pub enum NavAgentStrategy {
    FreeRoam,
    Navmesh,
}

impl Default for NavAgentStrategy {
    fn default() -> Self {
        NavAgentStrategy::FreeRoam
    }
}

#[derive(Component, Default)]
pub struct NavAgent {
    pub current: Option<Vec2>,
    pub path: Option<Vec<Vec2>>,
    pub strategy: NavAgentStrategy,
}

impl NavAgent {
    pub fn with_strategy(strategy: NavAgentStrategy) -> Self {
        NavAgent {
            strategy,
            ..Default::default()
        }
        // let mut nav_agent = NavAgent::default();
        // nav_agent.strategy = strategy;
        // nav_agent
    }
}

// pub fn click_pathfind_system(
//     mouse: Res<MouseState>,
//     mut er_mouse: EventReader<MouseButtonInput>,
//     q_navmesh: Query<(&Area, &NavMesh)>,
//     mut q_player: Query<(&mut NavAgent, &Parent, &Transform), With<Player>>,
// ) {
//     // TODO UPDATE fix this
//     // for e in er_mouse.iter() {
//     //     // if e.state != ElementState::Released || e.button != MouseButton::Left {
//     //     //     continue;
//     //     // }
//     //     let (mut player_agent, parent, player_trans) = q_player.single_mut().unwrap();
//     //     let (_, navmesh) = q_navmesh.get(parent.0).unwrap();
//     //     let player_pos = player_trans.translation.truncate();
//     //     player_agent.path = navmesh.find_path(player_pos, mouse.world_pos);
//     // }
// }

pub fn navagent_system(
    mut q_navagent: Query<(&mut NavAgent, &Movement, &mut Transform)>,
) {
    for (mut nav, movement, mut transform) in q_navagent.iter_mut() {
        if let Some(current) = nav.current {
            let diff = current - transform.translation.truncate();
            let dist = diff.length();
            let step = movement.level as f32 * SPEED_MULT; // * delta t
            if dist < f32::EPSILON {
                nav.current = None;
            } else if step > dist {
                // just move it to destination
                nav.current = None;
                transform.translation.x = current.x;
                transform.translation.y = current.y;
            } else {
                let diff_n = diff.normalize();
                transform.translation.x += diff_n.x * step;
                transform.translation.y += diff_n.y * step;
            }
        } else if let Some(path) = &mut nav.path {
            nav.current = path.pop();
        }
    }
}