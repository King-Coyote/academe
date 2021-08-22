use bevy::prelude::*;
use bevy_htn::prelude::*;
use rand::prelude::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(startup.system());
    }
}

#[derive(Default,)]
pub struct EnemyContext {
    state: ContextState,
    move_target: Vec2,
    current_pos: Vec2,
    move_timer_expired: bool,
}

impl Context for EnemyContext {
    fn state(&self) -> &ContextState { &self.state }
    fn state_mut(&mut self) -> &mut ContextState { &mut self.state }
    fn add(&mut self, key: &str, variant: Variant) { self.state.add(key, variant) }
    fn set(&mut self, key: &str, variant: Variant) { self.state.set(key, variant) }
    fn get(&self, key: &str) -> Option<&Variant> { self.state.get(key) }
    fn remove(&mut self, key: &str) { self.state.remove(key) }
    fn test_value(&self, key: &str, value: &Variant) -> Option<bool> { self.state.test_value(key, value) }
}

fn startup(
    mut commands: Commands,
) {
    let mut builder: BehaviourBuilder<EnemyContext> = BehaviourBuilder::new("Herbivore");
    builder
    .selector("BeEnemy")
        .primitive("MoveRandomly")
            .condition("The timer has expired", |ctx: &EnemyContext| ctx.move_timer_expired )
            .do_action("Choose new location", |ctx: &mut EnemyContext| -> TaskStatus {
                if ctx.move_target.abs_diff_eq(ctx.current_pos, f32::EPSILON) {
                    // make a new move loc
                    ctx.move_target = random_nearby_location(ctx.move_target);
                    return TaskStatus::Success;
                }
                TaskStatus::Continue
            })
        .end()
    .end()
    ;
    let behaviour = builder.build();
    behaviour.print();
    let mut planner = Planner::default();
    let mut ctx = EnemyContext::default();
    planner.tick(&behaviour, &mut ctx);
}

const MAX_MOVE_DISTANCE: f32 = 300.0;

fn random_nearby_location(p: Vec2) -> Vec2 {
    let mut rng = rand::thread_rng();
    let mid = MAX_MOVE_DISTANCE / 2.0;
    let dx: f32 = (MAX_MOVE_DISTANCE * rng.gen::<f32>()) - mid;
    let dy: f32 = (MAX_MOVE_DISTANCE * rng.gen::<f32>()) - mid;
    Vec2::new(
        p.x + dx,
        p.y + dy
    )
}