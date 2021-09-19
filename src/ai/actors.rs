use bevy_htn::{
    context::Context,
    prelude::{BehaviourBuilder, TaskStatus},
    task::TaskMacro,
};
use bevy::prelude::*;

#[derive(Default,)]
pub struct ActorStore {
    pub move_target: Option<Vec2>,
    pub current_pos: Vec2,
    pub wants_new_location: bool,
    pub current_time: f32,
}

pub trait ActorContext: Context {
    fn get_store(&self) -> &ActorStore;
    fn get_store_mut(&mut self) -> &mut ActorStore;
}

pub struct MoveRandomly;

impl<T> TaskMacro<T> for MoveRandomly 
where
    T: ActorContext
{
    fn build(&self, builder: &mut BehaviourBuilder<T>) {
        builder
        .primitive("MoveRandomly")
            .condition("The timer has expired", |ctx: &T| ctx.get_store().current_time > 4.0)
            .do_action("Choose new location", |ctx: &mut T| -> TaskStatus {
                let store = ctx.get_store_mut();
                if let Some(target) = store.move_target {
                    if target.abs_diff_eq(store.current_pos, f32::EPSILON) {
                        store.wants_new_location = true;
                        store.current_time = 0.0;
                        return TaskStatus::Success;
                    }
                }
                store.wants_new_location = false;
                TaskStatus::Continue
            })
        .end();
    }
}