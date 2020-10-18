use std::vec::Vec;
use std::collections::VecDeque;
use std::cmp::max;
use crate::ai::*;
use bevy::prelude::*;

pub enum TaskStatus {
    Continue,
    Success,
    Failure,
}

#[derive(Eq, PartialEq,)]
pub enum DecompositionStatus {
    Succeeded,
    Partial,
    Failed,
    Rejected,
}

impl Default for DecompositionStatus {
    fn default() -> Self {DecompositionStatus::Rejected}
}

pub enum ContextState {
    Planning,
    Executing,
}

impl Default for ContextState {
    fn default() -> Self {ContextState::Planning}
}
// snapshots the current planning done by the behaviour
#[derive(Default,)]
pub struct Record {

}

impl Record {
    pub fn is_empty(&self) -> bool {
        panic!("Not implemented")
    }

    pub fn clear(&mut self) {
        panic!("Not implemeneted")
    }
}

#[derive(Default,)]
pub struct WorldContext {
    // pub visible_enemies: Vec<Entity>,
    pub state: ContextState,
    pub record: Record,
    pub last_record: Record,
    pub partial_queue: VecDeque<usize>,
    pub paused: bool,
    pub dirty: bool,
}

// impl WorldContext {
//     pub fn new() -> Self {
//         WorldContext {
//             state: ContextState::Planning,
//             record: Record::default(),
            
//             dirty: false,
//             test: false,
//         }
//     }
// }

pub trait Condition {
    fn is_valid(&self, ctx: &WorldContext) -> bool;
}

impl<F> Condition for F 
where F: Fn(&WorldContext) -> bool {
    fn is_valid(&self, ctx: &WorldContext) -> bool {
        self(ctx)
    }
}

pub trait Operator {
    fn update(&self, ctx: &WorldContext) -> TaskStatus;
    fn stop(&self, ctx: &WorldContext);
}

pub trait Effect {
    fn apply(&self, ctx: &mut WorldContext);
}

impl<F> Effect for F
where F: Fn(&mut WorldContext) -> ()
{
    fn apply(&self, ctx: &mut WorldContext) {
        self(ctx)
    }
}

pub type Plan = VecDeque<usize>;