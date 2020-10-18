use crate::ai::*;
use std::mem;

pub struct Task {
    pub(super) name: String,
    pub(super) index: usize,
    pub(super) conditions: Vec<Box<dyn Condition>>,
    pub(super) exec_conditions: Vec<Box<dyn Condition>>, // conditions checked every execute
    pub(super) operator: Option<Box<dyn Operator>>,
    pub(super) effects: Vec<Box<dyn Effect>>,
    pub(super) parent: Option<usize>,
    pub(super) sub_tasks: Vec<usize>,
    pub(super) pausable: bool,
}

impl Task {
    pub fn new(name: &str, index: usize, parent: Option<usize>) -> Self {
        Task {
            name: name.to_owned(),
            index: index,
            conditions: vec![],
            exec_conditions: vec![],
            operator: None,
            effects: vec![],
            parent: parent,
            sub_tasks: vec![],
            pausable: false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        return self.sub_tasks.len() == 0
        && self.operator.is_none();
    }

    pub fn stop(&mut self, ctx: &WorldContext) {
        if self.operator.is_some() {
            self.operator.as_ref().unwrap().stop(ctx);
        }
    }

    pub(crate) fn add_child(&mut self, child: usize) {
        self.sub_tasks.push(child);
    }

    pub(crate) fn is_valid(&self, ctx: &WorldContext) -> bool {
        let mut valid = true;
        for cond in self.conditions.iter() {
            valid = valid && cond.is_valid(ctx);
        }
        valid
    }

    pub(crate) fn is_pausable(&self) -> bool {
        self.pausable
    }

    pub (crate) fn decompose(&mut self, ctx: &mut WorldContext, behaviour: &Behaviour, plan: &mut Plan) 
        -> DecompositionStatus
    {
        let decompositor = TaskDecomposition::new(self.index, ctx, behaviour);

        for subtask in self.sub_tasks.iter() {
            let mut task = behaviour.get_task(*subtask);
            let status = decompositor.decompose(&task, plan);
            match status {
                DecompositionStatus::Rejected
                | DecompositionStatus::Partial
                | DecompositionStatus::Failed => {
                    return status;
                },
                _ => {}
            };
        }

        decompositor.apply(plan);
        match plan.len() {
            l if l == 0 => DecompositionStatus::Failed,
            _ => DecompositionStatus::Succeeded
        }
    }
}


// represents a task decomposition in progress
struct TaskDecomposition<'s> {
    ctx: &'s mut WorldContext,
    behaviour: &'s Behaviour,
    calling_task: usize,
    plan: Plan,
    status: DecompositionStatus,
}

impl<'s> TaskDecomposition<'s> {
    pub fn new(calling: usize, ctx: &'s mut WorldContext, behaviour: &'s Behaviour) -> Self {
        TaskDecomposition {
            ctx: ctx,
            behaviour: behaviour,
            calling_task = calling,
            plan: Plan::default(),
            status: DecompositionStatus::default(),
        }
    }

    pub fn decompose(&mut self, task: &Task, over_plan: &mut Plan) -> DecompositionStatus {
        if !task.is_valid(self.ctx) {
            self.plan.clear();
            over_plan.clear();
            return DecompositionStatus::Failed;
        }

        // TODO replace this with a match on a task get_type function
        if task.is_pause() {
            self.ctx.paused = true;
            self.ctx.partial_queue.push_back(self.calling_task);
            self.apply(over_plan);
            return DecompositionStatus::Partial;
        } else {
            if !task.is_primitive() {
                return self.handle_compound(&mut self, task: &Task, over_plan: &mut Plan);
            } else {
                task.apply(ctx);
                self.plan.push_back(task.index);
            }
        }
        
        self.apply(over_plan);
        match over_plan.len() {
            l if l == 0 => DecompositionStatus::Failed,
            _ => DecompositionStatus::Succeeded
        }
    }

    pub fn handle_task(&mut self) -> DecompositionStatus {
        let mut status = DecompositionStatus::default();

        status
    }

    pub fn get_plan(self) -> Plan {
        self.plan
    }

    pub fn apply(self, plan: &mut Plan) {
        mem::swap(&mut self.plan, plan);
        self.plan.clear();
    }
}