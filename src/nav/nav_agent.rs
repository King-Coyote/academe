use bevy::{
    prelude::*,
    input::{
        ElementState,
        mouse::{MouseButtonInput, MouseButton},
    },
};
use crate::{
    game::*,
    input::*,
    nav::*,
    debug::*,
};

const SPEED_MULT: f32 = 5.0;

#[derive(Default)]
pub struct NavAgent {
    pub current: Option<Vec2>,
    pub path: Vec<Vec2>,
}

pub fn click_pathfind_system(
    mouse: Res<MouseState>,
    mut er_mouse: EventReader<MouseButtonInput>,
    q_navmesh: Query<&NavMesh>,
    mut q_player: Query<(&mut NavAgent, &Transform), With<Player>>,
) {
    for e in er_mouse.iter() {
        if e.state != ElementState::Released || e.button != MouseButton::Left {
            continue;
        }
        let (mut player_agent, player_trans) = q_player.single_mut().expect("There should be exactly 1 player!");
        let navmesh = q_navmesh.single().expect("Only allowing 1 navmesh rn");
        let player_pos = player_trans.translation.truncate();
        if let Some(path) = navmesh.find_path(player_pos, mouse.world_pos) {
            player_agent.path = path;
        }
    }
}

pub fn navagent_system(
    mut q_navagent: Query<(&mut NavAgent, &Movement, &mut Transform)>,
) {
    for (mut nav, movement, mut transform) in q_navagent.iter_mut() {
        if let Some(current) = nav.current {
            let diff = current - transform.translation.truncate();
            let dist = diff.length();
            let step = movement.level as f32 * SPEED_MULT; // * delta t
            if dist < f32::EPSILON {
                info!("Destination reached.");
                nav.current = None;
            } else if step > dist {
                info!("Step > dist: just going straight to the location.");
                // just move it to destination
                nav.current = None;
                transform.translation.x = current.x;
                transform.translation.y = current.y;
            } else {
                let diff_n = diff.normalize();
                transform.translation.x += diff_n.x * step;
                transform.translation.y += diff_n.y * step;
            }
        } else {
            nav.current = nav.path.pop();
        }
    }
}