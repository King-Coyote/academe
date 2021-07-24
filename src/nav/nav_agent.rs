use bevy::prelude::*;
use crate::game::*;

const SPEED_MULT: f32 = 1.0;

pub struct NavAgent {
    pub dest: Option<Vec2>, // Path later, probably
}

pub fn navagent_system(
    mut q_navagent: Query<(&mut NavAgent, &Movement, &mut Transform)>,
) {
    // todo: include dt in here
    for (mut nav, movement, mut transform) in q_navagent.iter_mut() {
        if nav.dest.is_none() {
            continue;
        }
        let destination = nav.dest.unwrap();
        let diff = destination - transform.translation.truncate();
        let dist = diff.length();
        let step = movement.level as f32 * SPEED_MULT; // * delta t
        if dist < f32::EPSILON {
            info!("Destination reached.");
            nav.dest = None;
        } else if step > dist {
            info!("Step > dist: just going straight to the location.");
            // just move it to destination
            nav.dest = None;
            transform.translation = destination.extend(destination.y);
        } else {
            info!("current pos is {}; interpolating towards pos {}", transform.translation, destination);
            let diff_n = diff.normalize();
            transform.translation.x += diff_n.x * step;
            transform.translation.y += diff_n.y * step;
        }
    }
}