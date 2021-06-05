use bevy::prelude::*;
use crate::{
    ui::*,
    utils::data_struct::*,
};

// game object things like despaw n cleanup

pub struct Despawning;

pub fn cleanup_despawned(
    mut commands: Commands,
    mut order: ResMut<InteractableOrder>,
    q_despawned: Query<Entity, With<Despawning>>,
    q_zindex: Query<&ZIndex>,
) {
    for entity in q_despawned.iter() {
        info!("Despawning entity {:?}...", entity);
        if let Ok(zindex) = q_zindex.get(entity) {
            info!("Found entity in z-order. Removing from that.");
            multimap_remove(&mut order.map, zindex.current, entity);
        }
        commands.entity(entity).despawn_recursive();
        info!("Finished despawning.");
    }
}