use std::collections::{VecDeque, HashMap,};
use bevy::ecs::Entity;

pub trait Context {
    // accessors
    fn is_dirty(&self) -> bool;
    fn set_dirty(&mut self, dirty: bool);
    fn is_paused(&self) -> bool;
    fn set_paused(&mut self, is_paused: bool);
    fn record(&mut self) -> &mut Record;
    fn last_record(&mut self) -> &mut Record;
    fn set_state(&mut self, new_state: ContextState);
    fn get_state(&self) -> ContextState;
    fn partial_queue(&mut self) -> &mut VecDeque<usize>;

    fn dump_into_record(&mut self);
    fn dump_into_last_record(&mut self);

    // this key must NOT exist before you add it
    fn add(&mut self, key: &str, variant: Variant);
    // doesn't care if the key exists
    fn set(&mut self, key: &str, variant: Variant);
    fn get(&self, key: &str) -> Option<&Variant>;
    fn remove(&mut self, key: &str);
    fn test_value(&self, key: &str, value: &Variant) -> Option<bool>;
    fn begin_transaction(&mut self);
    fn rollback_transaction(&mut self);
    fn commit_transaction(&mut self);
}

#[derive(Default,)]
pub struct BeingContext {
    pub(crate) state: ContextState,
    pub(crate) record: Record,
    pub(crate) last_record: Record,
    pub(crate) partial_queue: VecDeque<usize>,
    pub(crate) paused: bool,
    pub(crate) dirty: bool,
    world_state: HashMap<String, Variant>,
    transactions: Vec<Vec<String>>,
}

impl BeingContext {
    pub fn new() -> Self {
        BeingContext {
            dirty: true,
            ..Default::default()
        }
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

impl Context for BeingContext {

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn is_paused(&self) -> bool {
        self.paused
    }

    fn set_paused(&mut self, is_paused: bool) {
        self.paused = is_paused;
    }

    fn record(&mut self) -> &mut Record {
        &mut self.record
    }

    fn last_record(&mut self) -> &mut Record {
        &mut self.last_record
    }

    fn dump_into_record(&mut self) {
        self.record.clear();
        self.record.extend(&mut self.last_record);
    }

    fn dump_into_last_record(&mut self) {
        self.last_record.clear();
        self.last_record.extend(&mut self.record);
    }


    fn set_state(&mut self, new_state: ContextState) {
        self.state = new_state;
    }

    fn get_state(&self) -> ContextState {
        self.state
    }

    fn partial_queue(&mut self) -> &mut VecDeque<usize> {
        &mut self.partial_queue
    }

    // this key must NOT exist before you add it
    fn add(&mut self, key: &str, variant: Variant) {
        let last_value = self.world_state.insert(key.to_string(), variant);
        assert!(last_value.is_none());
        self.add_trans_key_if_needed(key);
    }

    // doesn't care if the key exists
    fn set(&mut self, key: &str, variant: Variant) {
        let last_value = self.world_state.insert(key.to_string(), variant);
        if last_value.is_none() {
            self.add_trans_key_if_needed(key);
        }
    }

    fn get(&self, key: &str) -> Option<&Variant> {
        self.world_state.get(key)
    }

    fn remove(&mut self, key: &str) {
        let last_value = self.world_state.remove(key);
        if last_value.is_some() {
            self.remove_trans_key_if_needed(key);
        }
    }

    fn test_value(&self, key: &str, value: &Variant) -> Option<bool> {
        if let Some(this_value) = self.get(key) {
            return Some(this_value == value)
        }   
        None
    }
    
    fn begin_transaction(&mut self) {
        self.transactions.push(vec![]);
    }

    fn rollback_transaction(&mut self) {
        assert!(self.transactions.len() > 0);
        for key in self.transactions.last().unwrap() {
            self.world_state.remove(key).expect("Rolled back a key that didn't exist - what on earth?! That should never happen.");
        }
        self.transactions.pop();
    }

    fn commit_transaction(&mut self) {
        assert!(self.transactions.len() > 0);
        self.transactions.pop();
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
        let mut ctx = BeingContext::new();
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
        let mut ctx = BeingContext::new();
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