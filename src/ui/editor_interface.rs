use bevy::{
    input::ElementState,
    prelude::*
};
use crate::{
    ui::*,
};

#[derive(Default,)]
pub struct PolygonBuilder {
    pub building: bool,
    pub first_point: Vec2,
    pub points: Vec<Vec2>,
    pub debug_ui: Option<Entity>,
}

// pub fn polygon_build_controls(
//     mut commands: Commands,
//     mouse: Res<MouseState>,
//     mut poly_build: ResMut<PolygonBuilder>,
//     mut er_mouse: EventReader<MouseButtonInput>,
//     key_input: Res<Input<KeyCode>>,
// ) {
//     // if key_input.just_released(KeyCode::Escape) {
//     //     poly_build.building = false;
//     // }
//     // for e in er_mouse.iter() {
//     //     if e.button != MouseButton::Left
//     //     || e.state != ElementState::Released {
//     //         continue;
//     //     }

//     //     if key_input.pressed(KeyCode::LControl) 
//     //     && !poly_build.building
//     //     {
//     //         poly_build.building = true;
//     //         poly_build.debug_ui = Some(commands.spawn().id());
//     //     }
//     //     if !poly_build.building {
//     //         continue;
//     //     }
//     //     let ui_entity = poly_build.debug_ui.unwrap();
//     //     if !poly_build.points.is_empty()
//     //     && mouse.world_pos.abs_diff_eq(*poly_build.points.get(0).unwrap(), 20.0) {
//     //         commands.entity(ui_entity).despawn();
//     //         poly_build.debug_ui = None;
//     //         // spawn a polygon here.
//     //     } else {
//     //         poly_build.points.push(mouse.world_pos);
//     //         let mut entity_build = commands.entity(ui_entity);
//     //         entity_build.insert(DebugCircleSpawn {
//     //             radius: 10.0,
//     //             center: mouse.world_pos,
//     //             color: Color::RED,
//     //         });
//     //         let num_points = poly_build.points.len();
//     //         if poly_build.points.len() > 1 {
//     //             entity_build.insert(DebugLineSpawn {
//     //                 origin: *poly_build.points.get(num_points - 1).unwrap(),
//     //                 dest: *poly_build.points.get(num_points).unwrap(),
//     //                 color: Color::RED,
//     //                 thickness: 2.0,
//     //             });
//     //         }
//     //     }
//     // }
// }