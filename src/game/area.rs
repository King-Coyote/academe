use bevy::{
    prelude::*,
};
use crate::{
    nav::*,
};

#[derive(Component)]
pub struct Area;

#[derive(Component)]
pub struct Loaded;

// pub fn area_system(
//     q_area: Query<(Entity, &Area, &Children, Option<&Loaded>)>,
// ) {

// }

pub fn area_loadstate_system(
    mut commands: Commands,
    q_area: Query<(Entity, &Area, &Children, Option<&Loaded>), Changed<Loaded>>,
    // q_area_children: Query<(Entity, &Parent, &Loaded)>,
) {
    for (_, _, children, loaded) in q_area.iter() {
        for child in children.iter() {
            if loaded.is_some() {
                commands.entity(*child).insert(Loaded);
            } else {
                commands.entity(*child).remove::<Loaded>();
            }
        }
    }
}