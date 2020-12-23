use std::collections::{VecDeque, HashMap,};
use bevy::ecs::Entity;

#[derive(Default,)]
pub struct WorldContext {
    pub(crate) state: ContextState,
    pub(crate) record: Record,
    pub(crate) last_record: Record,
    pub(crate) partial_queue: VecDeque<usize>,
    pub(crate) paused: bool,
    pub(crate) dirty: bool,
    world_state: HashMap<String, Variant>,
    transactions: Vec<Vec<String>>,
}

impl WorldContext {
    pub fn new() -> Self {
        WorldContext {
            dirty: true,
            ..Default::default()
        }
    }

    // this key must NOT exist before you add it
    pub fn add(&mut self, key: &str, variant: Variant) {
        let last_value = self.world_state.insert(key.to_string(), variant);
        assert!(last_value.is_none());
        self.add_trans_key_if_needed(key);
    }

    // doesn't care if the key exists
    pub fn set(&mut self, key: &str, variant: Variant) {
        let last_value = self.world_state.insert(key.to_string(), variant);
        if last_value.is_none() {
            self.add_trans_key_if_needed(key);
        }
    }

    pub fn get(&self, key: &str) -> Option<&Variant> {
        self.world_state.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        let last_value = self.world_state.remove(key);
        if last_value.is_some() {
            self.remove_trans_key_if_needed(key);
        }
    }

    pub fn test_value(&self, key: &str, value: &Variant) -> Option<bool> {
        if let Some(this_value) = self.get(key) {
            return Some(this_value == value)
        }   
        None
    }
    
    pub fn swap_records(&mut self) {}

    pub fn begin_transaction(&mut self) {
        self.transactions.push(vec![]);
    }

    pub fn rollback_transaction(&mut self) {
        assert!(self.transactions.len() > 0);
        for key in self.transactions.last().unwrap() {
            self.world_state.remove(key).expect("Rolled back a key that didn't exist - what on earth?! That should never happen.");
        }
        self.transactions.pop();
    }

    pub fn commit_transaction(&mut self) {
        assert!(self.transactions.len() > 0);
        self.transactions.pop();
    }

    fn add_trans_key_if_needed(&mut self, key: &str) {
        if let Some(transaction) = self.transactions.last_mut() {
            transaction.push(key.to_string());
        }
    }

    fn remove_trans_key_if_needed(&mut self, key: &str) {
        if let Some(transaction) = self.transactions.last() {
            if let Some(pos) = transaction.iter().position(|x| *x == key) {
                self.transactions.swap_remove(pos);
            }
        }
    }
}

// wrappings of various things that can exist in the game world
#[derive(Eq, PartialEq, Debug)]
pub enum Variant {
    Entity(Entity),
    Entities(Vec<Entity>),
    Location, // empty for now
    Bool(bool),
    Int32(i32),
}

// snapshots the current task planned by the behaviour
#[derive(Default,)]
pub struct Record {
    tasks: Vec<usize>,
}

impl Record {
    pub fn extend(&mut self, other: &mut Record) {
        self.tasks.extend(other.tasks.iter());
    }

    pub fn add(&mut self, task_index: usize) {
        self.tasks.push(task_index);
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.len() == 0
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }
}

pub enum ContextState {
    Planning,
    Executing,
}

impl Default for ContextState {
    fn default() -> Self {ContextState::Planning}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rollback_works() {
        let mut ctx = WorldContext::new();
        ctx.begin_transaction();
        ctx.set("test1", Variant::Int32(10));
        ctx.set("test2", Variant::Bool(true));
        ctx.rollback_transaction();

        assert!(ctx.world_state.get("test1").is_none());
        assert!(ctx.world_state.get("test2").is_none());
        assert!(ctx.transactions.len() == 0);

        ctx.set("test3", Variant::Int32(20));
        assert!(ctx.transactions.len() == 0);
    }

    #[test]
    fn commit_works() {
        let mut ctx = WorldContext::new();
        ctx.begin_transaction();
        ctx.set("test1", Variant::Int32(10));
        ctx.set("test2", Variant::Bool(true));
        ctx.commit_transaction();

        assert!(ctx.world_state.get("test1").is_some());
        assert!(ctx.world_state.get("test2").is_some());
        assert!(ctx.transactions.len() == 0);

        ctx.set("test3", Variant::Int32(20));
        assert!(ctx.transactions.len() == 0);
    }

}