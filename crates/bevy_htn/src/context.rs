use std::collections::{VecDeque, HashMap,};
use bevy::ecs::Entity;

pub trait Context {
    fn state(&self) -> &ContextState;
    fn state_mut(&mut self) -> &mut ContextState;
}

#[derive(Default,)]
pub struct ContextState {
    pub(crate) exec_state: ExecutionState,
    pub(crate) record: Record,
    pub(crate) last_record: Record,
    pub(crate) partial_queue: VecDeque<usize>,
    pub(crate) paused: bool,
    pub dirty: bool,
    vars: HashMap<String, Variant>,
    transactions: Vec<Vec<String>>,
}

impl ContextState {
    pub fn dump_into_record(&mut self) {
        self.record.clear();
        self.record.extend(&mut self.last_record);
    }

    pub fn dump_into_last_record(&mut self) {
        self.last_record.clear();
        self.last_record.extend(&mut self.record);
    }

    // this key must NOT exist before you add it
    pub fn add(&mut self, key: &str, variant: Variant) {
        let last_value = self.vars.insert(key.to_string(), variant);
        assert!(last_value.is_none());
        self.add_trans_key_if_needed(key);
    }

    // doesn't care if the key exists
    pub fn set(&mut self, key: &str, variant: Variant) {
        let last_value = self.vars.insert(key.to_string(), variant);
        if last_value.is_none() {
            self.add_trans_key_if_needed(key);
        }
    }

    pub fn get(&self, key: &str) -> Option<&Variant> {
        self.vars.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        let last_value = self.vars.remove(key);
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
    
    pub fn begin_transaction(&mut self) {
        self.transactions.push(vec![]);
    }

    pub fn rollback_transaction(&mut self) {
        assert!(self.transactions.len() > 0);
        for key in self.transactions.last().unwrap() {
            self.vars.remove(key).expect("Rolled back a key that didn't exist - what on earth?! That should never happen.");
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

// context for creatures, humans, etc
#[derive(Default,)]
pub struct BeingContext {
    state: ContextState,
}

impl BeingContext {
    pub fn new() -> Self {
        BeingContext {
            state: ContextState {
                dirty: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }

}

impl Context for BeingContext {

    fn state(&self) -> &ContextState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ContextState {
        &mut self.state
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

#[derive(Clone, Copy)]
pub enum ExecutionState {
    Planning,
    Executing,
}

impl Default for ExecutionState {
    fn default() -> Self {ExecutionState::Planning}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rollback_works() {
        let mut ctx = BeingContext::new();
        ctx.state_mut().begin_transaction();
        ctx.set("test1", Variant::Int32(10));
        ctx.set("test2", Variant::Bool(true));
        ctx.state_mut().rollback_transaction();

        assert!(ctx.world_state.get("test1").is_none());
        assert!(ctx.world_state.get("test2").is_none());
        assert!(ctx.transactions.len() == 0);

        ctx.set("test3", Variant::Int32(20));
        assert!(ctx.transactions.len() == 0);
    }

    #[test]
    fn commit_works() {
        let mut ctx = BeingContext::new();
        ctx.state_mut().begin_transaction();
        ctx.set("test1", Variant::Int32(10));
        ctx.set("test2", Variant::Bool(true));
        ctx.state_mut().commit_transaction();

        assert!(ctx.world_state.get("test1").is_some());
        assert!(ctx.world_state.get("test2").is_some());
        assert!(ctx.transactions.len() == 0);

        ctx.set("test3", Variant::Int32(20));
        assert!(ctx.transactions.len() == 0);
    }

}