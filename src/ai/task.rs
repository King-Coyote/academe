use crate::ai::*;

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
        let mut decomp = TaskDecomposition::new(ctx, plan);

        for sub_task in self.sub_tasks.iter() {
            let mut task = behaviour.get_task(*sub_task);

            if !task.is_valid(ctx) {
                return self.handle_invalid_task(task, ctx);
            }

            if !task.is_primitive() {
                return self.handle_compound_task(task, ctx);
            }

            if !task.is_pausable() {
                task.apply_effects(ctx);
                plan_status.0.push_back(*sub_task);
            } else {
                ctx.paused = true;
                ctx.partial_queue.push_back(self.index);
                plan_status.1 = DecompositionStatus::Partial;
                return plan_status;
            }

        }

        plan_status
    }

    fn handle_task_decomp(&mut self, behaviour: &Behaviour, decomp: &mut TaskDecomposition) {

    }
}

pub struct TaskDecomposition<'s> {
    ctx: &'s mut WorldContext,
    plan: &'s mut Plan,
    status: DecompositionStatus,
}

impl<'s> TaskDecomposition<'s> {
    pub fn new(ctx: &'s mut WorldContext, plan: &'s mut Plan) -> Self {
        TaskDecomposition {
            ctx: ctx,
            plan: plan,
            status: DecompositionStatus::default(),
        }
    }
}