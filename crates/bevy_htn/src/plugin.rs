use bevy::prelude::*;
use crate::prelude::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(startup.system());
    }
}

fn startup(
    mut commands: Commands,
) {
    let mut builder: BehaviourBuilder<BeingContext> = BehaviourBuilder::new("CreatureBehaviour");
    use Variant::*;
    builder
        .sequence("BeTestBehaviour")
            .sequence("FindEnemy")
                .condition("No enemies in range", |ctx: &BeingContext| {
                    !ctx.state().test_value("test", &Bool(true)).unwrap_or(false)
                })
                .selector("MoveRandomly")
                    .effect("Test!", |ctx: &mut BeingContext| ctx.state_mut().set("test", Bool(true)))
                    .do_action("TestOp", |ctx: &mut BeingContext| {
                        println!("I have many regrets; but the ass was fat");
                        TaskStatus::Success
                    })
                .end()
            .end()
            .sequence("MoveToEnemy")
                .do_action("TestOp2", |ctx: &mut BeingContext| {
                    println!("doing test op 2...");
                    TaskStatus::Continue
                })
            .end()
        .end();
    let behaviour = builder.build();
    behaviour.print();
    let mut planner = Planner::default();
    let mut ctx = BeingContext::new();
    planner.tick(&behaviour, &mut ctx);
    for _ in 1..10 {
        planner.tick(&behaviour, &mut ctx);
    }
}