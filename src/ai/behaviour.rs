use crate::ai::*;

pub struct Behaviour
    // where C: WorldContext
{   
    tasks: Vec<Task>,
    pub name: String,
}

impl Behaviour {
    pub fn new(name: &str, tasks: Vec<Task>) -> Self {
        assert!(tasks.len() > 0);
        Behaviour {
            tasks: tasks,
            name: name.to_owned(),
        }
    }

    pub fn get_task(&self, index: usize) -> &Task {
        self.tasks.get(index).expect("ERROR: wtf?? you tried to get a task from a behaviour at an index it doesn't have. This shouldn't happen!")
    }

    pub fn get_task_mut(&mut self, index: usize) -> &mut Task {
        self.tasks.get_mut(index).expect("ERROR: wtf?? you tried to get a task from a behaviour at an index it doesn't have. This shouldn't happen!")
    }

    pub fn find_plan(&self, ctx: &mut WorldContext) -> (Plan, DecompositionStatus) {
        ctx.state = ContextState::Planning;
        // let mut plan_status: (Plan, DecompositionStatus);
        let mut plan = Plan::default();
        let mut status = DecompositionStatus::default();

        if ctx.paused && ctx.record.is_empty() {
            ctx.paused = false;
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

        ctx.state = ContextState::Executing;
        (plan, status)
    }

    fn resume_partial(&self, ctx: &mut WorldContext, plan: &mut Plan) -> DecompositionStatus {
        let mut status = DecompositionStatus::default();
        if ctx.partial_queue.len() > 0 {
            let mut partial_task = &self.tasks[ctx.partial_queue.pop_front().unwrap()];
            status = partial_task.decompose(ctx, &self, plan);

            while ctx.partial_queue.len() > 0 && !ctx.paused {
                let mut new_plan = Plan::default();
                let mut new_status = DecompositionStatus::default();
                new_status = partial_task.decompose(ctx, &self, &mut new_plan);
                match new_status {
                    DecompositionStatus::Succeeded
                    | DecompositionStatus::Partial => {
                        plan.extend(&new_plan);
                    },
                    _ => {}
                }
            }
        }
        status
    }

    fn full_replan(&self, ctx: &mut WorldContext, plan: &mut Plan) -> DecompositionStatus {
        let mut last_partial_plan = Plan::default();
        let mut was_paused = false;
        if ctx.paused {
            ctx.paused = false;
            was_paused = true;
            last_partial_plan.extend(ctx.partial_queue.iter());
        }

        ctx.record.clear();
        let mut status = self.tasks[0].decompose(ctx, &self, plan);

        if was_paused {
            if status == DecompositionStatus::Rejected
            || status == DecompositionStatus::Failed {
                ctx.paused = true;
                ctx.partial_queue.clear();
                ctx.partial_queue.extend(last_partial_plan);
            }
        }

        status
    }

    // fn check_mtrs(&self, ctx: &mut WorldContext) -> DecompositionStatus {

    // }

    pub fn print(&self) {
        let mut stack: Vec<&Task> = vec![];
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

pub struct BehaviourBuilder<'s> {
    name: &'s str,
    current_task: Option<usize>,
    tasks: Vec<Task>,
    task_stack: Vec<usize>,
}

impl<'s> BehaviourBuilder<'s> {
    pub fn new(name: &'s str) -> Self {
        BehaviourBuilder {
            name: name,
            current_task: None,
            tasks: vec![],
            task_stack: vec![]
        }
    }

    pub fn task(&mut self, name: &str) -> &mut Self {
        let new_index: usize = self.tasks.len();
        self.tasks.push(Task::new(name, new_index, self.current_task));
        if let Some(index) = self.current_task {
            self.task_stack.push(index);
            self.tasks[index].add_child(new_index);
        }
        self.current_task = Some(new_index);
        self
    }

    pub fn condition<C: Condition + 'static>(&mut self, name: &str, condition: C) -> &mut Self {
        self.tasks[self.current_task.unwrap()]
            .conditions.push(Box::new(condition));
        self
    }

    pub fn effect<E: Effect + 'static>(&mut self, name: &str, effect: E) -> &mut Self {
        self.tasks[self.current_task.unwrap()]
            .effects.push(Box::new(effect));
        self
    }

    pub fn do_action<O: Operator + 'static>(&mut self, name: &str, operator: O) -> &mut Self {
        self.tasks[self.current_task.unwrap()]
            .operator = Some(Box::new(operator));
        self
    }

    pub fn end(&mut self) -> &mut Self {
        // pop task from stack
        self.current_task = self.task_stack.pop();
        self
    }

    pub fn pause(&mut self) -> &mut Self {
        self.tasks[self.current_task.unwrap()].pause = true;
        self
    }

    pub fn build(self) -> Behaviour {
        Behaviour::new(self.name, self.tasks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn behaviour_basic_init() {
        let mut builder: BehaviourBuilder = BehaviourBuilder::new("test");
        builder.task("test").end();
        let behaviour = builder.build();

        assert!(behaviour.tasks.len() == 1);
        assert_eq!(behaviour.name, "test");
    }

    #[test]
    fn add_subtask_works() {
        let mut builder: BehaviourBuilder = BehaviourBuilder::new("test");
        builder
            .task("test_parent")
                .task("test_child")
                .end()
            .end();
        let behav = builder.build();
        assert_eq!(behav.tasks.len(), 2);
        assert_eq!(behav.tasks[0].sub_tasks.len(), 1);
    }
}