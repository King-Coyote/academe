use crate::prelude::*;
use crate::task::*;
use std::collections::VecDeque;
use std::marker::PhantomData;

#[derive(Default,)]
pub struct Planner<C> 
    where C: Context
{
    plan: Plan,
    current_task: Option<usize>,
    last_status: TaskStatus,
    pd: PhantomData<C>,
}

impl<C> Planner<C> 
    where C: Context
{
    pub fn tick(&mut self, behaviour: &Behaviour<C>, ctx: &mut C) {

        let mut status = DecompositionStatus::Failed;
        let mut replacing = false;

        // get plan if we need it
        if !self.has_plan() && self.current_task.is_none() || ctx.state_mut().dirty {
            replacing = self.plan.len() > 0;
            status = self.find_plan(ctx, behaviour);
        }

        // get current task from plan if needed
        if self.has_plan() && self.current_task.is_none() {
            self.get_task_from_plan(ctx, behaviour);
        }

        // handle the current task
        if let Some(task) = self.current_task {
            let task_ref = behaviour.get_task(task);
            if task_ref.get_type() == TaskType::Primitive {
                self.handle_task(ctx, task_ref);
            }
        }

        // handle failure
        if !self.has_plan()
        && self.current_task.is_none()
        && !replacing
        && (
            status == DecompositionStatus::Failed 
            || status == DecompositionStatus::Rejected
        )
        {
            self.last_status = TaskStatus::Failure;
        }
    }

    fn find_plan(&mut self, ctx: &mut C, behaviour: &Behaviour<C>) 
        -> DecompositionStatus
    {

        let dirty = ctx.state_mut().dirty;
        ctx.state_mut().dirty = false;
        let mut last_partial_plan: VecDeque<usize> = VecDeque::new();

        if dirty && ctx.state_mut().paused {
            //replan
            ctx.state_mut().paused = false;
            last_partial_plan.extend(ctx.state_mut().partial_queue.iter());
            ctx.state_mut().dump_into_last_record();
        }

        let plan_status = behaviour.find_plan(ctx);
        match plan_status.1 {
            DecompositionStatus::Succeeded
            | DecompositionStatus::Partial => {
                self.plan.clear();
                self.plan.extend(plan_status.0);

                // are we currently on a primitive task?
                if let Some(task_index) = self.current_task {
                    let task = behaviour.get_task(task_index);
                    if task.task_type != TaskType::Primitive {
                        task.stop(ctx);
                        self.current_task = None;
                    }
                }

                ctx.state_mut().dump_into_last_record();

            },
            _ => {
                if last_partial_plan.len() > 0 {
                    ctx.state_mut().paused = true;
                    ctx.state_mut().partial_queue.clear();
                    ctx.state_mut().partial_queue.extend(last_partial_plan);

                    if !ctx.state_mut().last_record.is_empty() {
                        ctx.state_mut().dump_into_record();
                        ctx.state_mut().last_record.clear();
                    }
                }
            }
        };
        
        plan_status.1
    }

    fn get_task_from_plan(&mut self, ctx: &mut C, behaviour: &Behaviour<C>) {
        let current = self.plan.pop_front().unwrap();
        self.current_task = Some(current);
        let task_ref = behaviour.get_task(current);
        for condition in task_ref.conditions.iter() {
            if !condition.is_valid(ctx) {
                self.clear_all(ctx);
            }
        }
    }

    fn handle_task(&mut self, ctx: &mut C, task: &Task<C>) {
        match &task.operator {
            Some(ref op) => {
                for exec_cond in task.exec_conditions.iter() {
                    if !exec_cond.is_valid(ctx) {
                        self.clear_all(ctx);
                        return;
                    }
                }
                self.last_status = op.update(ctx);
                match self.last_status {
                    TaskStatus::Success => {
                        // I don't actually reckon I need this tnbh, only for planning
                        // for effect in task.effects.iter() {
                        //     effect.apply(ctx);
                        // }
                        self.current_task = None;
                        if self.plan.len() == 0 {
                            ctx.state_mut().last_record.clear();
                            ctx.state_mut().dirty = false;
                            // call tick again if immediate replanning is required
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

    pub fn has_plan(&self) -> bool {
        self.plan.len() > 0
    }

    fn clear_all(&mut self, ctx: &mut C) {
        self.current_task = None;
        self.plan.clear();
        ctx.state_mut().last_record.clear();
        ctx.state_mut().paused = false;
        ctx.state_mut().partial_queue.clear();
        ctx.state_mut().dirty = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planner_has_no_plan_at_start() {
        let planner = Planner::<BeingContext>::default();
        assert_eq!(planner.current_task, None);
        assert!(planner.plan.len() == 0);
    }

    #[test]
    #[should_panic]
    fn tick_empty_behaviour_expectedbehav() {
        let mut ctx = BeingContext::default();
        let mut builder = BehaviourBuilder::new("test");
        let b = builder.build();
        let mut p = Planner::default();
        p.tick(&b, &mut ctx);
    }

    #[test]
    fn tick_primitive_no_operator_expectedbehav() {
        let mut ctx = BeingContext::default();
        let mut builder = BehaviourBuilder::new("test");
        builder
            .sequence("super")
                .primitive("primitive")
                .end()
            .end();
        let b = builder.build();
        let mut p = Planner::default();

        p.tick(&b, &mut ctx);

        assert_eq!(p.current_task, None);
        assert_eq!(p.last_status, TaskStatus::Failure);
    }

    #[test]
    fn tick_with_empty_func_operator_expectedbehav() {
        let mut ctx = BeingContext::default();
        let mut builder = BehaviourBuilder::new("test");
        builder
            .selector("super")
                .primitive("primitive")
                    .do_action("test", |ctx: &mut BeingContext| {TaskStatus::Success})
                .end()
            .end();
        let b = builder.build();
        let mut p = Planner::default();

        p.tick(&b, &mut ctx);

        assert_eq!(p.last_status, TaskStatus::Success);
    }

}