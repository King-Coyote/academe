
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
