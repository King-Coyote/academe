use std::{
    cmp::Ord,
    collections::{btree_map::Entry, BTreeMap,},
};
use bevy::prelude::*;

pub type MultiTreeMap<K, V> = BTreeMap<K, Vec<V>>;

pub fn multimap_insert<K, V>(map: &mut MultiTreeMap<K, V>, k: K, v: V)
where
    K: Ord,
{
    map.entry(k).or_insert_with(Vec::new).push(v);
}

pub fn multimap_remove<K, V>(map: &mut MultiTreeMap<K, V>, k: K, v: V)
where
    K: Ord,
    V: Eq,
{
    if let Entry::Occupied(mut entry) = map.entry(k) {
        entry.get_mut().retain(|val| *val != v);
        if entry.get().is_empty() {
            entry.remove();
        }
    }
}

pub fn decode_vec2(vec: &Vec2) -> (u32, u32) {
    (
        (vec.x as f32).to_bits(),
        (vec.y as f32).to_bits()
    )
}