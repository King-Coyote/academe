use bevy::prelude::*;
use crate::game::*;

const SPEED_MULT: f32 = 5.0;

#[derive(Default)]
pub struct NavAgent {
    pub current: Option<Vec2>,
    pub path: Vec<Vec2>,
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
                info!("current pos is {}; interpolating towards pos {}", transform.translation, current);
                let diff_n = diff.normalize();
                transform.translation.x += diff_n.x * step;
                transform.translation.y += diff_n.y * step;
            }
        } else {
            nav.current = nav.path.pop();
            if let Some(current) = nav.current {
                info!("new popped pos: {}", current);
            }
        }
    }
}