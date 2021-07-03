use std::{
    cmp::Ord,
    collections::{btree_map::Entry, BTreeMap,}
};

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
