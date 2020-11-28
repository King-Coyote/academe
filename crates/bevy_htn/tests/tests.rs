
use bevy_htn::prelude::*;

#[test]
fn basic_planning_works() {
    let mut builder = BehaviourBuilder::new("test");
    builder
        .task("task1")
            .task("subtask1")
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
        .task("task1")
            .task("subtask1")
            .end()
        .end();
    let mut b = builder.build();
    let mut p = Planner::default();
    let mut ctx = WorldContext::default();

    p.tick(&b, &mut ctx);

    // now assert what?
}
