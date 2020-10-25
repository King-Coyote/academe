use bevy::prelude::*;
use rlua::prelude::*;
use crate::ai::*;

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
    let mut builder: BehaviourBuilder = BehaviourBuilder::new("CreatureBehaviour");
    builder
        .task("BeTestBehaviour")
            .task("FindEnemy")
                .condition("No enemies in range", |ctx: &WorldContext| !ctx.test)
                .task("MoveRandomly")
                    .effect("Test!", |ctx: &mut WorldContext| ctx.test = true)
                    .do_action("TestOp", || {
                        println!("I have many regrets; but the ass was fat");
                        TaskStatus::Success
                    })
                .end()
            .end()
            .task("MoveToEnemy")
                .do_action("TestOp2", || {
                    println!("doing test op 2...");
                    TaskStatus::Continue
                })
            .end()
        .end();
    let behaviour = builder.build();
    behaviour.print();
    let mut planner = Planner::default();
    let mut ctx = WorldContext::new();
    planner.tick(&behaviour, &mut ctx);
    for _ in 1..10 {
        planner.tick(&behaviour, &mut ctx);
    }
}