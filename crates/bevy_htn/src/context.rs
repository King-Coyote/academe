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
}

impl WorldContext {
    pub fn new() -> Self {
        WorldContext {
            dirty: true,
            ..Default::default()
        }
    }

    pub fn set(&mut self, tag: &str, variant: Variant) {
        self.world_state.insert(tag.to_string(), variant);
    }

    pub fn get(&self, tag: &str) -> Option<&Variant> {
        self.world_state.get(tag)
    }

    pub fn test_value(&self, tag: &str, value: &Variant) -> Option<bool> {
        if let Some(this_value) = self.get(tag) {
            return Some(this_value == value)
        }   
        None
    }
    
    pub fn swap_records(&mut self) {}
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

// snapshots the current planning done by the behaviour
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