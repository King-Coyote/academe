use crate::prelude::*;
use crate::task::*;
use std::marker::PhantomData;

pub struct Behaviour<C>
    where C: Context
{   
    tasks: Vec<Task<C>>,
    pub name: String,
    pd: PhantomData<C>,
}

impl<C: Context> Behaviour<C> {
    pub fn new(name: &str, tasks: Vec<Task<C>>) -> Behaviour<C> {
        assert!(tasks.len() > 0);
        Behaviour::<C> {
            tasks: tasks,
            name: name.to_owned(),
            pd: PhantomData::default(),
        }
    }

    pub fn get_task(&self, index: usize) -> &Task<C> {
        self.tasks.get(index).expect("ERROR: wtf?? you tried to get a task from a behaviour at an index it doesn't have. This shouldn't happen!")
    }

    pub fn get_task_mut(&mut self, index: usize) -> &mut Task<C> {
        self.tasks.get_mut(index).expect("ERROR: wtf?? you tried to get a task from a behaviour at an index it doesn't have. This shouldn't happen!")
    }

    pub fn find_plan(&self, ctx: &mut C) -> (Plan, DecompositionStatus) {
        ctx.set_state(ContextState::Planning);
        // let mut plan_status: (Plan, DecompositionStatus);
        let mut plan = Plan::default();
        let mut status = DecompositionStatus::default();

        if ctx.is_paused() && ctx.last_record().is_empty() {
            ctx.set_paused(false);
            status = self.resume_partial(ctx, &mut plan);
        } else {
            status = self.full_replan(ctx, &mut plan);
        }

        // check if mtrs are same for optimisation
        // plan_status.1 = self.check_mtrs(ctx);
        // match status {
        //     DecompositionStatus::Succeeded
        //     | DecompositionStatus::Partial => {

        //     },
        //     _ => {

        //     }
        // }

        ctx.set_state(ContextState::Executing);
        (plan, status)
    }

    fn resume_partial(&self, ctx: &mut C, plan: &mut Plan) -> DecompositionStatus
        where C: Context
    {
        use DecompositionStatus::*;

        let mut status = DecompositionStatus::default();
        let mut decomposed = false;
        while ctx.partial_queue().len() > 0 && !ctx.is_paused() {
            let partial_task = &self.tasks[ctx.partial_queue().pop_front().unwrap()];
            if !decomposed {
                status = partial_task.decompose(ctx, &self, plan);
                decomposed = true;
            } else {
                let mut new_plan = Plan::new();
                status = partial_task.decompose(ctx, &self, &mut new_plan);
                if status == Succeeded || status == Partial {
                        plan.extend(new_plan.iter());
                }
            }
        }

        // we failed to continue the paused partial plan, so we replan from root.
        if status == Rejected || status == Partial {
            ctx.record().clear();
            status = self.get_task(0).decompose(ctx, &self, plan);
        }

        status
    }

    fn full_replan(&self, ctx: &mut C, plan: &mut Plan) -> DecompositionStatus {
        let mut last_partial_plan = Plan::default();
        let mut was_paused = false;
        if ctx.is_paused() {
            ctx.set_paused(false);
            was_paused = true;
            last_partial_plan.extend(ctx.partial_queue().iter());
        }

        ctx.record().clear();
        let mut status = self.tasks[0].decompose(ctx, &self, plan);

        use DecompositionStatus::*;
        if was_paused && (status == Rejected || status == Failed) {
            ctx.set_paused(true);
            ctx.partial_queue().clear();
            ctx.partial_queue().extend(last_partial_plan);
        }

        status
    }

    // fn check_mtrs(&self, ctx: &mut BeingContext) -> DecompositionStatus {

    // }

    pub fn print(&self) {
        let mut stack: Vec<&Task<C>> = vec![];
        for task in self.tasks.iter().rev() {
            if task.parent.is_none() {
                stack.push(&task);
            }
        }
        let mut num_tabs = 0;
        while stack.len() > 0 {
            let current = stack.pop().unwrap();
            println!("{:indent$}{name}", "", indent=num_tabs, name=current.name);
            for child_index in current.sub_tasks.iter().rev() {
                stack.push(&self.tasks[*child_index]);
            }
            if current.sub_tasks.len() == 0 {
                if num_tabs >= 1 {
                    num_tabs -= 1;
                }
            } else {
                num_tabs += 1;
            }
        }
    }
}

pub struct BehaviourBuilder<'s, C> 
    where C: Context
{
    name: &'s str,
    current_task: Option<usize>,
    tasks: Vec<Task<C>>,
    task_stack: Vec<usize>,
    pd: PhantomData<C>,
}

impl<'s, C: Context> BehaviourBuilder<'s, C> {
    pub fn new(name: &'s str) -> Self {
        BehaviourBuilder::<C> {
            name: name,
            current_task: None,
            tasks: vec![],
            task_stack: vec![],
            pd: PhantomData::default(),
        }
    }

    pub fn selector(&mut self, name: &str) -> &mut Self {
        self.create_task(name, TaskType::Selector);
        self
    }

    pub fn sequence(&mut self, name: &str) -> &mut Self {
        self.create_task(name, TaskType::Sequence);
        self
    }

    pub fn primitive(&mut self, name: &str) -> &mut Self {
        self.create_task(name, TaskType::Primitive);
        self
    }
    pub fn condition<K: Condition<C> + 'static>(&mut self, name: &str, condition: K) -> &mut Self {
        self.tasks[self.current_task.unwrap()]
            .conditions.push(Box::new(condition));
        self
    }

    pub fn effect<E: Effect<C> + 'static>(&mut self, name: &str, effect: E) -> &mut Self {
        self.tasks[self.current_task.unwrap()]
            .effects.push(Box::new(effect));
        self
    }

    pub fn do_action<O: Operator<C> + 'static>(&mut self, name: &str, operator: O) -> &mut Self {
        self.tasks[self.current_task.unwrap()]
            .add_operator(Box::new(operator));
        self
    }

    pub fn end(&mut self) -> &mut Self {
        // pop task from stack
        self.current_task = self.task_stack.pop();
        self
    }

    pub fn pause(&mut self) -> &mut Self {
        self.create_task("", TaskType::Pause);
        self
    }

    pub fn build(self) -> Behaviour<C> {
        Behaviour::<C>::new(self.name, self.tasks)
    }

    fn create_task(&mut self, name: &str, task_type: TaskType) {
        let new_index: usize = self.tasks.len();
        self.tasks.push(Task::<C>::new(name, new_index, self.current_task, task_type));
        if let Some(index) = self.current_task {
            self.task_stack.push(index);
            self.tasks[index].add_child(new_index);
        }
        self.current_task = Some(new_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn behaviour_basic_init() {
        let mut builder: BehaviourBuilder<BeingContext> = BehaviourBuilder::new("test");
        builder.selector("test").end();
        let behaviour = builder.build();

        assert!(behaviour.tasks.len() == 1);
        assert_eq!(behaviour.name, "test");
    }

    #[test]
    fn add_subtask_works() {
        let mut builder: BehaviourBuilder<BeingContext> = BehaviourBuilder::new("test");
        builder
            .sequence("test_parent")
                .sequence("test_child")
                .end()
            .end();
        let behav = builder.build();
        assert_eq!(behav.tasks.len(), 2);
        assert_eq!(behav.tasks[0].sub_tasks.len(), 1);
    }

    #[test]
    #[should_panic]
    fn adding_operator_to_compound_fails() {
        let mut builder: BehaviourBuilder<BeingContext> = BehaviourBuilder::new("test");
        builder
            .selector("test_parent")
                .do_action("durr", |ctx: &mut BeingContext| {TaskStatus::Success})
            .end();
        let behav = builder.build();
    }

    #[test]
    #[should_panic]
    fn adding_task_to_primitive_fails() {
        let mut builder: BehaviourBuilder<BeingContext> = BehaviourBuilder::new("test");
        builder
            .primitive("test_parent")
                .selector("durr")
                .end()
            .end();
        let behav = builder.build();
    }
}