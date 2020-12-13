use crate::prelude::*;
use std::mem;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TaskType {
    Sequence,
    Selector,
    Primitive,
    Pause,
}

pub struct Task {
    pub(super) name: String,
    pub(super) index: usize,
    pub(super) conditions: Vec<Box<dyn Condition>>,
    pub(super) exec_conditions: Vec<Box<dyn Condition>>, // conditions checked every execute
    pub(super) operator: Option<Box<dyn Operator>>,
    pub(super) effects: Vec<Box<dyn Effect>>,
    pub(super) parent: Option<usize>,
    pub(super) sub_tasks: Vec<usize>,
    pub(super) task_type: TaskType,
}

impl Task {
    pub fn new(name: &str, index: usize, parent: Option<usize>, task_type: TaskType) -> Self {
        Task {
            name: name.to_owned(),
            index: index,
            conditions: vec![],
            exec_conditions: vec![],
            operator: None,
            effects: vec![],
            parent: parent,
            sub_tasks: vec![],
            task_type: task_type,
        }
    }

    pub fn stop(&self, ctx: &WorldContext) {
        if self.operator.is_some() {
            self.operator.as_ref().unwrap().stop(ctx);
        }
    }

    pub fn get_type(&self) -> TaskType {
        self.task_type
    }

    pub(crate) fn add_child(&mut self, child: usize) {
        assert!(self.task_type != TaskType::Primitive);
        self.sub_tasks.push(child);
    }

    pub(crate) fn is_valid(&self, ctx: &WorldContext) -> bool {
        let mut valid = true;
        for cond in self.conditions.iter() {
            valid = valid && cond.is_valid(ctx);
        }
        valid
    }

    pub fn apply_effects(&self, ctx: &mut WorldContext) {
        for effect in self.effects.iter() {
            effect.apply(ctx);
        }
    }

    pub (crate) fn add_operator(&mut self, operator: Box<dyn Operator>) {
        assert!(self.task_type == TaskType::Primitive);
        self.operator = Some(operator);
    }

    pub (crate) fn decompose(&self, ctx: &mut WorldContext, behaviour: &Behaviour, plan: &mut Plan) 
        -> DecompositionStatus
    {
        let mut decompositor = TaskDecomposition::new(self.index, ctx, behaviour);

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
            calling_task: calling,
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

        match task.get_type() {
            TaskType::Selector | TaskType::Sequence => {
                return self.decompose_sequence(task, over_plan);
            },
            TaskType::Pause => {
                return self.decompose_pausable(over_plan);
            },
            TaskType::Primitive => {
                // decompose primitive
                task.apply_effects(self.ctx);
                self.plan.push_back(task.index);
            },
        }
        
        self.apply_to(over_plan);
        match over_plan.len() {
            l if l == 0 => DecompositionStatus::Failed,
            _ => DecompositionStatus::Succeeded
        }
    }

    fn decompose_sequence(&mut self, task: &Task, over_plan: &mut Plan) -> DecompositionStatus {
        let mut sub_plan = Plan::default();
        match task.decompose(self.ctx, self.behaviour, &mut sub_plan) {
            DecompositionStatus::Rejected
            | DecompositionStatus::Failed => {
                // this is a bit different in fluid htn - it nulls the over_plan
                // this may cause issues 4 u
                self.plan.clear();
                // ctx.trim??
                over_plan.clear();
                return DecompositionStatus::Rejected;
            },
            _ => {}
        }

        over_plan.extend(sub_plan);

        if self.ctx.paused {
            self.ctx.partial_queue.push_back(task.index);
            self.apply_to(over_plan);
            return DecompositionStatus::Partial;
        }

        DecompositionStatus::Succeeded
    }

    fn decompose_selector(&mut self, task: &Task, over_plan: &mut Plan) -> DecompositionStatus {
        
    }

    fn decompose_pausable(&mut self, plan: &mut Plan) -> DecompositionStatus {
        self.ctx.paused = true;
        self.ctx.partial_queue.push_back(self.calling_task);
        self.apply_to(plan);
        return DecompositionStatus::Partial;
    }

    pub fn apply_to(&mut self, plan: &mut Plan) {
        mem::swap(&mut self.plan, plan);
        self.plan.clear();
    }
}