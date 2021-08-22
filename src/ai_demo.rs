use bevy::prelude::*;
use bevy_htn::prelude::*;
use rand::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::collections::HashMap;
use std::ops;
use std::f32;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugin(ShapePlugin)
        .add_stage_before(stage::UPDATE, "ai_planning", SystemStage::parallel())
        .add_startup_system(startup.system())
        .add_system_to_stage("ai_planning", ai_system.system())
        .add_system(senses_system.system())
        .add_system(movement_system.system())
        .add_system(ai_system.system())
        ;
    }
}

#[derive(Default,)]
pub struct CreatureContext {
    state: ContextState,
    move_target: Option<Vec2>, // if it's not None, nav should pick this up and go with it
    current_pos: Vec2,
    move_timer_expired: bool,
    nearby_herbivore: Vec<Entity>,
    nearby_carnivore: Vec<Entity>,
    nearest_carnivore: Option<(Entity, f32)>,
}

impl Context for CreatureContext {
    fn state(&self) -> &ContextState { &self.state }
    fn state_mut(&mut self) -> &mut ContextState { &mut self.state }
    fn add(&mut self, key: &str, variant: Variant) { self.state.add(key, variant) }
    fn set(&mut self, key: &str, variant: Variant) { self.state.set(key, variant) }
    fn get(&self, key: &str) -> Option<&Variant> { self.state.get(key) }
    fn remove(&mut self, key: &str) { self.state.remove(key) }
    fn test_value(&self, key: &str, value: &Variant) -> Option<bool> { self.state.test_value(key, value) }
}

fn startup(
    world: &mut World, 
    resources: &mut Resources
) {

    let herbivore_behaviour = herbivore_behaviour();
    let carnivore_behaviour = carnivore_behaviour();
    let mut behaviour_map = BehaviourMap::default();
    behaviour_map.behaviours.insert("Herbivore".to_string(), herbivore_behaviour);
    behaviour_map.behaviours.insert("Carnivore".to_string(), carnivore_behaviour);
    resources.insert_thread_local(behaviour_map);

    let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();

    let materials = Materials {
        carn_mat: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        herb_mat: materials.add(Color::rgb(0.0, 0.75, 0.2).into()),
    };

    println!("Spawning herbivores...");

    for i in 0..4 {
        spawn_herbivore(
            world,
            materials.herb_mat.clone(),
            Vec2::new(i as f32 * 10.0, 0.0),
            10.0
        );
    }

    spawn_carnivore(world, materials.carn_mat.clone(), Vec2::new(-300.0, 0.0), 12.0);

    world.spawn(Camera2dBundle::default());
    // let behaviour = builder.build();
    // behaviour.print();
    // let mut planner = Planner::default();
    // let mut ctx = CreatureContext::default();
    // planner.tick(&behaviour, &mut ctx);

}

fn ai_system(
    world: &mut World, 
    resources: &mut Resources
) {
    let behaviour_map = resources.get_thread_local::<BehaviourMap>().unwrap();
    for (mut ctx, mut planner, name) in world.query_mut::<(&mut CreatureContext, &mut Planner<CreatureContext>, &BehaviourName)>() {
        let behaviour = behaviour_map.behaviours.get(&name.0).unwrap();
        planner.tick(behaviour, &mut *ctx);
    }
}

fn senses_system(
    mut ctx_query: Query<(&mut CreatureContext, &Transform)>,
    mut herb_query: Query<(&Herbivore, &Transform, Entity)>,
    mut carn_query: Query<(&Carnivore, &Transform, Entity)>
) {
    for (mut ctx, trans) in ctx_query.iter_mut() {
        ctx.nearby_herbivore.clear();
        let prev_nearest_carnivore = ctx.nearest_carnivore;
        ctx.nearest_carnivore = None;
        ctx.nearby_carnivore.clear();
        for (_, other_trans, entity) in herb_query.iter() {
            if within_sight(trans, other_trans) {
                ctx.nearby_herbivore.push(entity);
            }
        }
        for (_, other_trans, entity) in carn_query.iter() {
            let mut closest = f32::INFINITY;
            if within_sight(trans, other_trans) {
                let dist = (trans.translation - other_trans.translation).length();
                if dist < closest {
                    closest = dist;
                    ctx.nearest_carnivore = Some((entity, dist));
                }
                ctx.nearby_carnivore.push(entity);
            }
        }
        if prev_nearest_carnivore != ctx.nearest_carnivore {
            // force replan if you done seen more enemies
            ctx.move_target = None;
            ctx.state.dirty = true;
        }
    }
}

fn movement_system(
    time: Res<Time>,
    mut move_query: Query<(&mut Movement, &mut Transform, &mut CreatureContext)>
) {
    for (mut movement, mut trans, mut ctx) in move_query.iter_mut() {
        if let Some(ref target) = ctx.move_target {
            let level = movement.level as f32;
            movement.velocity = move_velocity_towards(
                &movement.velocity, 
                &ctx.current_pos, 
                target, 
                level * time.delta_seconds()
            );
            trans.translation.x += movement.velocity.x;
            trans.translation.y += movement.velocity.y;
            ctx.current_pos.x = trans.translation.x;
            ctx.current_pos.y = trans.translation.y;
        }
    }
}

#[derive(Default)]
struct BehaviourMap {
    behaviours: HashMap<String, Behaviour<CreatureContext>>,
}

struct Materials {
    carn_mat: Handle<ColorMaterial>,
    herb_mat: Handle<ColorMaterial>,
}

struct Herbivore;
struct Carnivore;
struct DebugCircle;
struct BehaviourName(String);

struct Movement{
    level: u32,
    velocity: Vec2,
}

impl Movement {
    fn new(level: u32) -> Self {
        Movement{
            level: level,
            velocity: Vec2::new(0.0, 0.0)
        }
    }
}

const MAX_MOVE_DISTANCE: f32 = 150.0;
const BOUNDARY: f32 = 400.0;
const MOVE_MULTIPLIER: f32 = 25.0;
const HERBIVORE_SIGHT_RANGE: f32 = 200.0;

fn spawn_herbivore(
    world: &mut World,
    material: Handle<ColorMaterial>,
    pos: Vec2,
    r: f32,
) {
    let circle = shapes::Circle {
        radius: r,
        ..Default::default()
    };

    let e = world.spawn(
        GeometryBuilder::build_as(
            &circle,
            material,
            TessellationMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(pos[0], pos[1], 0.0)),
        )
    );
    world.insert(e, (
        DebugCircle,
        Herbivore,
        Movement::new(1),
        CreatureContext::default(),
        Planner::<CreatureContext>::default(),
        BehaviourName("Herbivore".to_owned()),
    )).unwrap();
}

fn herbivore_behaviour() -> Behaviour<CreatureContext> {
    let mut builder: BehaviourBuilder<CreatureContext> = BehaviourBuilder::new("Herbivore");
    builder
    .selector("BeAHerbivore")
        .primitive("RunAwayFromEnemies")
            .condition("HasNearestEnemy", |ctx: &CreatureContext| ctx.nearest_carnivore.is_some())
            .do_action("Run away", |ctx: &mut CreatureContext| -> TaskStatus {
                // check the distance that the enemy has
                // if the distance is under some constant, then you need to keep moving away
                // 
                println!("I see a carnivore!!");
                TaskStatus::Success
            })
        .end()
        .primitive("MoveRandomly")
            .do_action("Choose new location", |ctx: &mut CreatureContext| -> TaskStatus {
                match ctx.move_target {
                    Some(ref target) => {
                        if locations_close(target, &ctx.current_pos) {
                            // make a new move loc
                            ctx.move_target = Some(random_nearby_location(&ctx.current_pos));
                            println!("Destination reached: now moving to {:?}", ctx.move_target.unwrap());
                            return TaskStatus::Success;
                        }
                    },
                    _ => {
                        ctx.move_target = Some(random_nearby_location(&ctx.current_pos));
                        println!("No move target: moving to {:?}", ctx.move_target.unwrap());
                        return TaskStatus::Success;
                    }
                }
                TaskStatus::Continue
            })
        .end()
    .end();
    builder.build()
}

fn carnivore_behaviour() -> Behaviour<CreatureContext> {
    let mut builder: BehaviourBuilder<CreatureContext> = BehaviourBuilder::new("Carnivore");
    builder
    .selector("BeCarnivore")
        // .primitive("AttackHerbivore")
        //     .condition("HasNearestHerbivore", |ctx: &CreatureContext| -> ctx.nearest_herbivore.is_some())
        //     .do_action("AttackHerbivore", |ctx: &mut CreatureContext| -> TaskStatus {

        //         TaskStatus::Success
        //     })
        // .end()
        .primitive("MoveRandomly")
            .do_action("Choose new location", |ctx: &mut CreatureContext| -> TaskStatus {
                match ctx.move_target {
                    Some(ref target) => {
                        if locations_close(target, &ctx.current_pos) {
                            // make a new move loc
                            ctx.move_target = Some(random_nearby_location(&ctx.current_pos));
                            return TaskStatus::Success;
                        }
                    },
                    _ => {
                        ctx.move_target = Some(random_nearby_location(&ctx.current_pos));
                        return TaskStatus::Success;
                    }
                }
                TaskStatus::Continue
            })
        .end()
    .end();
    builder.build()
}

fn spawn_carnivore(
    world: &mut World,
    material: Handle<ColorMaterial>,
    pos: Vec2,
    r: f32,
) {
    let circle = shapes::Circle {
        radius: r,
        ..Default::default()
    };

    let e = world.spawn(
        GeometryBuilder::build_as(
            &circle,
            material,
            TessellationMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(pos[0], pos[1], 0.0)),
        )
    );
    world.insert(e, (
        DebugCircle,
        Carnivore,
        Movement::new(2),
        CreatureContext::default(),
        Planner::<CreatureContext>::default(),
        BehaviourName("Carnivore".to_owned()),
    )).unwrap();
}

fn locations_close(a: &Vec2, b: &Vec2) -> bool {
    a.abs_diff_eq(*b, 1.0)
    // let dx = a.x - b.x;
    // let dy = a[1] - b[1];
    // (dx.powi(2) + dy.powi(2)).sqrt() <= f32::EPSILON
}

fn within_sight(a: &Transform, b: &Transform) -> bool {
    a.translation.distance(b.translation) < HERBIVORE_SIGHT_RANGE
}

fn location_away_from(current: &Vec2, from: &Vec2) -> Vec2 {
    let diff = (*current - *from).normalize();
    diff * MAX_MOVE_DISTANCE * -1.0
}

fn random_nearby_location(p: &Vec2) -> Vec2 {
    let mut rng = rand::thread_rng();
    let mid = MAX_MOVE_DISTANCE / 2.0;
    let dx: f32 = clamp_abs((MAX_MOVE_DISTANCE * rng.gen::<f32>()) - mid, BOUNDARY);
    let dy: f32 = clamp_abs((MAX_MOVE_DISTANCE * rng.gen::<f32>()) - mid, BOUNDARY);
    Vec2::new(p.x + dx, p.y + dy)
}

fn move_point_towards(from: &Vec2, to: &Vec2, amount: f32) -> Vec2 {
    from.lerp(*to, amount * MOVE_MULTIPLIER)
}

fn move_velocity_towards(velocity: &Vec2, pos: &Vec2, target: &Vec2, amount: f32) -> Vec2 {
    let dir = (*target - *pos).normalize();
    let max_vel = dir * amount * MOVE_MULTIPLIER;
    if velocity.abs_diff_eq(max_vel, f32::EPSILON) {
        return velocity.clone();
    }
    velocity.lerp(max_vel, 0.5)
}

fn clamp_abs(mut a: f32, to: f32) -> f32 {
    if a > to { return to; }
    if a < to * -1.0 { return to * -1.0; }
    a
}

fn clamp(n: f32, low: f32, high: f32) -> f32 {
    if n < low { return low; }
    if n > high { return high; }
    n
}

fn length(v: &[f32; 2]) -> f32 {
    (v[0].powi(2) + v[1].powi(2)).sqrt()
}