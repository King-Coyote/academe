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
pub struct HerbivoreContext {
    state: ContextState,
    move_target: [f32; 2],
    current_pos: [f32; 2],
    move_timer_expired: bool,
}

impl Context for HerbivoreContext {
    fn state(&self) -> &ContextState { &self.state }
    fn state_mut(&mut self) -> &mut ContextState { &mut self.state }
    fn add(&mut self, key: &str, variant: Variant) { self.state.add(key, variant) }
    fn set(&mut self, key: &str, variant: Variant) { self.state.set(key, variant) }
    fn get(&self, key: &str) -> Option<&Variant> { self.state.get(key) }
    fn remove(&mut self, key: &str) { self.state.remove(key) }
    fn test_value(&self, key: &str, value: &Variant) -> Option<bool> { self.state.test_value(key, value) }
}

fn startup(
    mut commands: &mut Commands,
) {
    let mut builder: BehaviourBuilder<HerbivoreContext> = BehaviourBuilder::new("Herbivore");
    use Variant::*;
    builder
    .selector("BeAHerbivore")
        .primitive("MoveRandomly")
            .condition("The timer has expired", |ctx: &HerbivoreContext| ctx.move_timer_expired )
            .do_action("Choose new location", |ctx: &mut HerbivoreContext| -> TaskStatus {
                if locations_close(&ctx.move_target, &ctx.current_pos) {
                    // make a new move loc
                    ctx.move_target = random_nearby_location(&ctx.move_target);
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
    let mut ctx = HerbivoreContext::default();
    planner.tick(&behaviour, &mut ctx);
}

const EPSILON: f32 = 0.001;

fn locations_close(a: &[f32; 2], b: &[f32; 2]) -> bool {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    (dx.powi(2) + dy.powi(2)).sqrt() <= EPSILON
}

const MAX_MOVE_DISTANCE: f32 = 300.0;

fn random_nearby_location(p: &[f32; 2]) -> [f32; 2] {
    let mut rng = rand::thread_rng();
    let mid = MAX_MOVE_DISTANCE / 2.0;
    let dx: f32 = (MAX_MOVE_DISTANCE * rng.gen::<f32>()) - mid;
    let dy: f32 = (MAX_MOVE_DISTANCE * rng.gen::<f32>()) - mid;
    [
        p[0] + dx,
        p[1] + dy
    ]
}