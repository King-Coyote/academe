
use bevy_htn::prelude::*;

#[test]
fn basic_planning_works() {
    let mut builder = BehaviourBuilder::new("test");
    builder
        .sequence("sequence1")
            .sequence("subsequence1")
            .end()
        .end();
    let mut b = builder.build();
    let mut p = Planner::default();
    let mut ctx = WorldContext::default();

    p.tick(&b, &mut ctx);

    // now assert what?
}

#[test]
fn more_complex_planning_works() {
    let mut builder = BehaviourBuilder::new("test");
    builder
        .sequence("sequence1")
            .sequence("subsequence1")
            .end()
        .end();
    let mut b = builder.build();
    let mut p = Planner::default();
    let mut ctx = WorldContext::default();

    p.tick(&b, &mut ctx);

    // now assert what?
}

#[test]
fn selector_continues_after_failed_task() {
    use Variant::*;

    let mut builder = BehaviourBuilder::new("test");
    builder
    .selector("sel1")
        .sequence("i will fail")
            .primitive("Base_False")
                .condition("always_false", |ctx: &WorldContext| false)
                .do_action("set_wrong_flag", |ctx: &mut WorldContext| {
                    ctx.set("wrong", Bool(true));
                    return TaskStatus::Success;
                })
            .end()
        .end()
        .selector("i should be chosen")
            .primitive("Base_Real")
                .do_action("set_flag", |ctx: &mut WorldContext| {
                    ctx.set("test", Bool(true));
                    return TaskStatus::Success;
                })
            .end()
        .end()
    .end();
    let mut b = builder.build();
    let mut p = Planner::default();
    let mut ctx = WorldContext::default();

    p.tick(&b, &mut ctx);
    assert!(ctx.test_value("wrong", &Bool(true)).is_none());
    assert_eq!(ctx.get("test").unwrap(), &Bool(true));
}


#[test]
fn sequence_with_failed_task_gives_no_plan() {
    use Variant::*;

    let mut builder = BehaviourBuilder::new("test");
    builder
    .sequence("i should give no plan")
        .primitive("I should run successfully")
            .do_action("durr", |ctx: &mut WorldContext| {
                ctx.set("yeh", Bool(true));
                TaskStatus::Success
            })
        .end()
        .primitive("I should also run")
            .do_action("hurr", |ctx: &mut WorldContext| {
                ctx.set("yeh m8 lmfao", Bool(true));
                TaskStatus::Success
            })
        .end()
        .primitive("I should fail")
            .condition("always invalid", |ctx: &WorldContext| {
                ctx.test_value("nothing", &Bool(true)).is_some()
            })
            .do_action("nah", |ctx: &mut WorldContext| {
                ctx.set("nahhh", Bool(true));
                TaskStatus::Success
            })
        .end()
    .end();
    let mut b = builder.build();
    let mut p = Planner::default();
    let mut ctx = WorldContext::default();

    p.tick(&b, &mut ctx);
    assert!(!p.has_plan());
    assert!(ctx.get("yeh").is_none());
    assert!(ctx.get("hurr").is_none());
    assert!(ctx.get("nahhh").is_none());
}

#[test]
fn failed_sequence_does_not_pollute_context() {
    use Variant::*;

    let mut builder = BehaviourBuilder::new("test");
    builder
    .sequence("i should fail but not pollute")
        .selector("Some of mine run")
            .primitive("I should run successfully")
                .effect("pollute", |ctx: &mut WorldContext| {
                    ctx.set("pollution", Bool(true))
                })
            .end()
            .primitive("I should run successfully too")
                .effect("pollute2", |ctx: &mut WorldContext| {
                    ctx.set("pollution2", Bool(true))
                })
            .end()
        .end()
        .primitive("I should fail")
            .condition("always fails", |ctx: &WorldContext| false)
        .end()
    .end();
    let mut b = builder.build();
    let mut p = Planner::default();
    let mut ctx = WorldContext::default();

    p.tick(&b, &mut ctx);
    assert!(ctx.get("pollution").is_none());
    assert!(ctx.get("pollution2").is_none());
}
