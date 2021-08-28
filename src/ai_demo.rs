use std::collections::HashMap;
use bevy::prelude::*;
use bevy_htn::prelude::*;
use rand::prelude::*;
use crate::{NavAgent, NavMesh};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .insert_resource(BehaviourMap::default())
        .add_startup_system(startup.system())
        .add_system(ai_system.system())
        .add_system(senses_system.system())
        ;
    }
}

#[derive(Default,)]
pub struct EnemyContext {
    pub name: String,
    pub state: ContextState,
    pub move_target: Option<Vec2>,
    pub current_pos: Vec2,
    pub wants_new_location: bool,
    pub current_time: f32,
}

#[derive(Default)]
struct BehaviourMap {
    pub behaviours: HashMap<String, Behaviour<EnemyContext>>,
    // pub behaviours: HashMap<String, i32>,
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
    mut behaviour_map: ResMut<BehaviourMap>,
    mut commands: Commands,
) {
    info!("Setting up enemy behaviour!");
    let mut builder: BehaviourBuilder<EnemyContext> = BehaviourBuilder::new("Herbivore");
    builder
    .selector("BeEnemy")
        .primitive("MoveRandomly")
            .condition("The timer has expired", |ctx: &EnemyContext| ctx.current_time > MOVE_TIMEOUT )
            .do_action("Choose new location", |ctx: &mut EnemyContext| -> TaskStatus {
                if let Some(target) = ctx.move_target {
                    if target.abs_diff_eq(ctx.current_pos, f32::EPSILON) {
                        info!("current time is {}!", ctx.current_time);
                        ctx.wants_new_location = true;
                        ctx.current_time = 0.0;
                        return TaskStatus::Success;
                    }
                }
                ctx.wants_new_location = false;
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
    behaviour_map.behaviours.insert("BeEnemy".to_string(), behaviour);
}

fn ai_system(
    assets: ResMut<Assets<ColorMaterial>>,
    behaviour_map: Res<BehaviourMap>,
    q_navmesh: Query<&NavMesh>,
    mut q_ai: Query<(&mut EnemyContext, &mut Planner<EnemyContext>, &mut NavAgent)>,
) {
    for (mut ctx, mut planner, mut nav) in q_ai.iter_mut() {
        let behaviour = behaviour_map.behaviours.get(&ctx.name).unwrap();
        planner.tick(behaviour, &mut *ctx);
        if ctx.wants_new_location {
            let navmesh = q_navmesh.single().expect("There should be exactly 1 navmesh");
            let curr = ctx.current_pos;
            loop {
                let dest = random_nearby_location(curr);
                if let Some(path) = navmesh.find_path(curr, dest) {
                    ctx.move_target = Some(dest);
                    nav.path = path;
                    ctx.wants_new_location = false;
                    break;
                }
            }
        }
    }
}

fn senses_system(
    time: Res<Time>,
    mut q_context: Query<(Entity, &mut EnemyContext, &Transform)>,
) {
    for (entity, mut context, transform) in q_context.iter_mut() {
        context.current_time += time.delta_seconds();
        context.current_pos = transform.translation.truncate();
    }
}

const MAX_MOVE_DISTANCE: f32 = 700.0;
const MOVE_TIMEOUT: f32 = 4.0;

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