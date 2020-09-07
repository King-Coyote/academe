use bevy::prelude::*;

// Tiles can only be added if they have an owning space
pub struct Space {
    size: (u32, u32),
}

struct Tile {
    // owning_space: Entity,
}