use crate::ai::*;
use std::collections::VecDeque;

struct Planner {
    plan: Plan,
    current_task: Option<usize>,
    last_status: TaskStatus,
}

impl Planner {
    pub fn tick(&mut self, behaviour: &Behaviour, ctx: &mut WorldContext) {

        // first bool is whether this plan is replacing another.
        let mut plan_status: (bool, Plan, DecompositionStatus) = 
            (false, Plan::default(), DecompositionStatus::default());
        let mut replacing_plan = false;

        // get plan if we need it
        if !self.has_plan() && self.current_task.is_none() || ctx.dirty {
            // NOTE you need to actually do a find plan for the planner here
            // that USES the domain find_plan but also handles replacing_plan
            plan_status = self.find_plan(ctx, behaviour);
        }

        // get current task from plan if needed
        if self.has_plan() && self.current_task.is_none() {
            self.get_task_from_plan(ctx, behaviour);
        }

        // handle the current task
        if let Some(task) = self.current_task {
            let task_ref = behaviour.get_task(task);
            if task_ref.is_primitive() {
                self.handle_task(ctx, task_ref);
            }
        }

        // handle failure
        if !self.has_plan()
        && self.current_task.is_none()
        && !replacing_plan
        && (
            plan_status.2 == DecompositionStatus::Failed 
            || plan_status.2 == DecompositionStatus::Rejected
        )
        {
            self.last_status = TaskStatus::Failure;
        }
    }

    fn find_plan(&mut self, ctx: &mut WorldContext, behaviour: &Behaviour) 
        -> (bool, Plan, DecompositionStatus)
    {
        let mut plan_status = (Plan::default(), DecompositionStatus::default());
        let mut replacing = false;
        let dirty = ctx.dirty;
        ctx.dirty = false;
        let mut last_partial_plan: VecDeque<usize> = VecDeque::new();

        if dirty && ctx.paused {
            //replan
            ctx.paused = false;
            last_partial_plan.extend(ctx.partial_queue);
            ctx.last_record.clear();
            ctx.swap_records();
        }

        let plan_status = behaviour.find_plan(ctx);
        // are we trying to replace an existing plan?
        replacing = self.plan.len() > 0;
        match plan_status.1 {
            DecompositionStatus::Succeeded
            | DecompositionStatus::Partial => {
                self.plan.clear();
                self.plan.extend(plan_status.0);

                // are we currently on a primitive task?
                if let Some(task_index) = self.current_task {
                    let task = behaviour.get_task(task_index);
                    if task.is_primitive() {
                        task.stop(ctx);
                        self.current_task = None;
                    }
                }

                ctx.last_record.clear();
                ctx.swap_records();

            },
            _ => {
                if last_partial_plan.len() > 0 {
                    ctx.paused = true;
                    ctx.partial_queue.clear();
                    ctx.partial_queue.extend(last_partial_plan);

                    if !ctx.last_record.is_empty() {
                        ctx.record.clear();
                        ctx.swap_records();
                        ctx.last_record.clear();
                    }
                }
            }
        }
        
        (replacing, plan_status.0, plan_status.1)
    }

    fn get_task_from_plan(&mut self, ctx: &mut WorldContext, behaviour: &Behaviour) {
        let current = self.plan.pop_front().unwrap();
        self.current_task = Some(current);
        let task_ref = behaviour.get_task(current);
        for condition in task_ref.conditions {
            if !condition.is_valid(ctx) {
                self.clear_all(ctx);
            }
        }
    }

    fn handle_decomp_success(&mut self, ctx: &WorldContext, new_plan: &Plan) {
        self.plan.clear();
        self.plan = new_plan;
        if let Some(task) = self.current_task {
            if task.is_primitive() {
                task.stop(ctx);
                self.current_task = None;
            }
        }

        // clear decomp record and etc
    }

    fn handle_task(&mut self, ctx: &mut WorldContext, task: &Task) {
        match task.operator {
            Some(op) => {
                for exec_cond in task.exec_conditions {
                    if !exec_cond.is_valid(ctx) {
                        self.clear_all(ctx);
                        return;
                    }
                }
                self.last_status = op.update(ctx);
                match self.last_status {
                    TaskStatus::Success => {
                        for effect in task.effects.iter() {
                            effect.apply(ctx);
                        }
                        self.current_task = None;
                        if self.plan.len() == 0 {
                            // clear ctx decomps
                            ctx.dirty = false;
                        }
                    },
                    TaskStatus::Failure => {
                        self.clear_all(ctx);
                    },
                    _ => {} // continue current task
                }
            },
            None => {
                // shouldn't really get here - if so, you may have set your behaviour up wrong
                println!("Root task found with no operator! That is silly.");
                self.current_task = None;
                self.last_status = TaskStatus::Failure;
            }
        }
    }

    fn has_plan(&self) -> bool {
        self.plan.len() > 0
    }

    fn clear_all(&mut self, ctx: &mut WorldContext) {
        self.current_task = None;
        self.plan.clear();
        // clear decomp history for context
        // clear partial plan history for context
        ctx.dirty = false;
    }
}