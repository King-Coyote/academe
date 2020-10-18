use crate::ai::*;

pub struct Behaviour
    // where C: WorldContext
{   
    tasks: Vec<Task>,
    pub name: String,
}

impl Behaviour {
    pub fn new(name: &str, tasks: Vec<Task>) -> Self {
        Behaviour {
            tasks: tasks,
            name: name.to_owned(),
        }
    }

    pub fn get_task(&self, index: usize) -> &Task {
        self.tasks.get(index).expect("ERROR: wtf?? you tried to get a task from a behaviour at an index it doesn't have. This shouldn't happen!")
    }

    pub fn find_plan(&self, ctx: &mut WorldContext) -> (Plan, DecompositionStatus) {
        ctx.state = ContextState::Planning;
        // let mut plan_status: (Plan, DecompositionStatus);
        let mut plan = Plan::default();
        let mut status = DecompositionStatus::default();
        // let mut plan: Plan;
        // let mut status: DecompositionStatus;

        if ctx.paused && ctx.record.is_empty() {
            ctx.paused = false;
            plan_status = self.resume_partial(ctx);
        } else {
            plan_status = self.full_replan(ctx);
        }

        // check if mtrs are same for optimisation
        // plan_status.1 = self.check_mtrs(ctx);
        match plan_status.1 {
            DecompositionStatus::Succeeded
            | DecompositionStatus::Partial => {

            },
            _ => {

            }
        }

        ctx.state = ContextState::Executing;
        (plan, status)
    }

    fn resume_partial(&self, ctx: &mut WorldContext, plan: &mut Plan) -> DecompositionStatus {
        let mut status = DecompositionStatus::default();
        if ctx.partial_queue.len() > 0 {
            let mut partial_task = &self.tasks[ctx.partial_queue.pop_front().unwrap()];
            status = partial_task.decompose(ctx, plan);

            while ctx.partial_queue.len() > 0 && !ctx.paused {
                let mut new_plan = Plan::default();
                let mut new_status = DecompositionStatus::default();
                new_status = partial_task.decompose(ctx, new_plan);
                match new_status {
                    DecompositionStatus::Succeeded
                    | DecompositionStatus::Partial => {
                        plan.take_all(&new_plan);
                    },
                    _ => {}
                }
            }
        }
        status
    }

    fn full_replan(&self, ctx: &mut WorldContext, plan: &mut Plan) -> DecompositionStatus {
        let mut last_partial_plan: VecDeque<usize> = VecDeque::new();
        if ctx.paused {
            ctx.paused = false;
            last_partial_plan.extend(ctx.partial_queue);
        }

        ctx.record.clear();
        let mut status = self.tasks[0].decompose(ctx, plan);

        if last_partial_plan.len() > 0 {
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
        let task = &mut self.tasks[self.current_task.unwrap()]
            .conditions.push(Box::new(condition));
        self
    }

    pub fn effect<E: Effect + 'static>(&mut self, name: &str, effect: E) -> &mut Self {
        let task = &mut self.tasks[self.current_task.unwrap()]
            .effects.push(Box::new(effect));
        self
    }

    pub fn end(&mut self) -> &mut Self {
        // pop task from stack
        self.current_task = self.task_stack.pop();
        self
    }

    pub fn pausable(&mut self) -> &mut Self {
        self.tasks[self.current_task.unwrap()].pausable = true;
        self
    }

    pub fn build(self) -> Behaviour {
        Behaviour::new(self.name, self.tasks)
    }
}