use std::collections::VecDeque;

#[derive(Eq, PartialEq, Debug)]
pub enum TaskStatus {
    Continue,
    Success,
    Failure,
}

impl Default for TaskStatus {
    fn default() -> Self {TaskStatus::Failure}
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

#[derive(Default,)]
pub struct WorldContext {
    // pub visible_enemies: Vec<Entity>,
    pub state: ContextState,
    pub record: Record,
    pub last_record: Record,
    pub partial_queue: VecDeque<usize>,
    pub paused: bool,
    pub dirty: bool,
    pub test: bool,
}

impl WorldContext {
    pub fn new() -> Self {
        WorldContext {
            dirty: true,
            test: false,
            ..Default::default()
        }
    }
    
    pub fn swap_records(&mut self) {}
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
    fn update(&self) -> TaskStatus;
    fn stop(&self, ctx: &WorldContext);
}

impl<F> Operator for F
where F: Fn() -> TaskStatus 
{
    fn update(&self) -> TaskStatus {
        self()
    }

    fn stop(&self, ctx: &WorldContext) {

    }
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