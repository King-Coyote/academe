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
        .task("FindEnemy")
            .condition("No enemies in range", |ctx: &WorldContext| !ctx.test)
            .task("MoveRandomly")
                .effect("", |ctx: &mut WorldContext| ctx.test = true)
            .end()
        .end()
        .task("MoveToEnemy")
        .end();
    let behaviour = builder.build();
    behaviour.print();
}