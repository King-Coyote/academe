use crate::prelude::*;

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

    pub fn stop(&self, ctx: &mut WorldContext) {
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
        let mut decomposition = TaskDecomposition::new(self.index, ctx, behaviour);
        decomposition.decompose(&self, plan)
    }
}


struct TaskDecomposition<'s> {
    ctx: &'s mut WorldContext,
    behaviour: &'s Behaviour,
    calling_task: usize,
}

impl<'s> TaskDecomposition<'s> {
    pub fn new(calling: usize, ctx: &'s mut WorldContext, behaviour: &'s Behaviour) -> Self {
        TaskDecomposition {
            ctx: ctx,
            behaviour: behaviour,
            calling_task: calling,
        }
    }

    pub fn decompose(&mut self, task: &Task, over_plan: &mut Plan) -> DecompositionStatus {
        use TaskType::*;

        match task.get_type() {
            Sequence => {
                return self.decompose_sequence(task, over_plan);
            },
            Selector => {
                return self.decompose_selector(task, over_plan);
            }
            Pause => {
                return self.decompose_pause(over_plan);
            },
            Primitive => {
                return self.decompose_primitive(task, over_plan);
            },
        }
    
    }

    fn decompose_sequence(&mut self, task: &Task, over_plan: &mut Plan) -> DecompositionStatus {
        use DecompositionStatus::*;

        let mut sub_plan = Plan::default();
        self.ctx.begin_transaction();
        for sub_task_inx in task.sub_tasks.iter() {
            let sub_task = self.behaviour.get_task(*sub_task_inx);
            if !task.is_valid(self.ctx) {
                sub_plan.clear();
                return Failed;
            }
            let status = sub_task.decompose(self.ctx, self.behaviour, &mut sub_plan);
            match status {
                Rejected | Failed | Partial => {
                    self.ctx.rollback_transaction();                    
                    return status;
                },
                _ => {}
            }
        }

        match sub_plan.len() {
            l if l == 0 => Failed,
            _ => {
                self.ctx.commit_transaction();
                over_plan.extend(sub_plan.iter());
                Succeeded
            }
        }
    }

    fn decompose_selector(&mut self, task: &Task, over_plan: &mut Plan) -> DecompositionStatus {
        use DecompositionStatus::*;

        let mut sub_plan = Plan::default();
        // TODO check if current task we're about to decompose
        // can possibly beat the running plan
        for sub_task_inx in task.sub_tasks.iter() {
            let sub_task = self.behaviour.get_task(*sub_task_inx);
            if !task.is_valid(self.ctx) {
                over_plan.extend(sub_plan.iter());
                return Failed
            }
            let status = sub_task.decompose(self.ctx, self.behaviour, &mut sub_plan);
            match status {
                Rejected | Succeeded | Partial => {
                    over_plan.extend(sub_plan.iter());
                    return status;
                },
                _ => continue
            }
        }

        Failed

        // probably don't need this...
        // match sub_plan.len() {
        //     l if l == 0 => Failed,
        //     _ => {
        //         over_plan.extend(sub_plan.iter());
        //         Succeeded
        //     }
        // }
    }

    fn decompose_pause(&mut self, plan: &mut Plan) -> DecompositionStatus {
        use DecompositionStatus::*;

        self.ctx.paused = true;
        self.ctx.partial_queue.push_back(self.calling_task);
        
        Partial
    }

    fn decompose_primitive(&mut self, task: &Task, over_plan: &mut Plan) -> DecompositionStatus {
        use DecompositionStatus::*;

        if !task.is_valid(self.ctx) {
            return Failed
        }

        task.apply_effects(self.ctx);
        over_plan.push_back(task.index);
        Succeeded
    }

    // pub fn apply_to(&mut self, over_plan: &mut Plan) {
    //     mem::swap(&mut self.plan, over_plan);
    //     self.plan.clear();
    // }
}

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