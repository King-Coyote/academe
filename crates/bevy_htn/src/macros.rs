use crate::prelude::*;

#[macro_export]
macro_rules! test_task {
    ($name:literal) => {
        BehaviourBuilder::new($name)
        .selector($name)
            .primitive("dur")
                .do_action("print", |ctx: &mut WorldContext| {
                    println!("test macro success!");
                    TaskStatus::Success
                })
            .end()
        .end()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_macro() {
        let mut ctx = WorldContext::default();
        let mut builder = BehaviourBuilder::new("test");
        test_task!("testyboi");
        let b = builder.build();
    }
}