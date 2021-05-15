use std::iter::Filter;

use bevy::{
    prelude::*,
    ecs::query::{
        WorldQuery,
        FilterFetch,
        ReadOnlyFetch,
    },
};

pub fn children_match_query<Q, F>(
    children: &Children, 
    query: &Query<Q, F>) -> bool
where
    Q: WorldQuery,
    F: WorldQuery,
    F::Fetch: FilterFetch,
    Q::Fetch: ReadOnlyFetch,
{
    children.iter().any(|c| query.get(*c).is_ok())
}