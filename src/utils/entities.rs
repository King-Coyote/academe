use bevy::{
    ecs::query::{ReadFetch, WorldQuery},
    prelude::*,
};

pub fn children_match_query<Q, F>(children: &Children, query: &Query<Q, F>) -> bool
where
    Q: WorldQuery,
    F: WorldQuery,
{
    children.iter().any(|c| query.get(*c).is_ok())
}

// pub fn do_for_children<Q, F, A>(children: &Children, query: &mut Query<Q, F>, mut action: A) 
// where
//     Q: WorldQuery,
//     F: WorldQuery,
//     F::Fetch: FilterFetch,
//     A: FnMut(<Q::Fetch as bevy::ecs::query::Fetch>::Item)
// {
//     for entity in children.iter() {
//         if let Ok(child) = query.get_mut(*entity) {
//             action(child);
//         }
//     }
// }