use bevy::prelude::*;
use bevy_htn::prelude::*;
use rand::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::collections::HashMap;

pub struct AiPlugin;

type Position = [f32; 2];

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
    move_target: Option<Position>, // if it's not None, nav should pick this up and go with it
    current_pos: Position,
    move_timer_expired: bool,
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
    let mut builder: BehaviourBuilder<CreatureContext> = BehaviourBuilder::new("Herbivore");
    use Variant::*;
    builder
    .selector("BeAHerbivore")
        .primitive("MoveRandomly")
            .do_action("Choose new location", |ctx: &mut CreatureContext| -> TaskStatus {
                match ctx.move_target {
                    Some(ref target) => {
                        if locations_close(target, &ctx.current_pos) {
                            // make a new move loc
                            ctx.move_target = Some(random_nearby_location(&ctx.current_pos));
                            println!("Reached destination: choosing a new location: {:?}", &ctx.move_target);
                            return TaskStatus::Success;
                        }
                    },
                    _ => {
                        ctx.move_target = Some(random_nearby_location(&ctx.current_pos));
                        println!("No move target - making one: {:?}", &ctx.move_target);
                        return TaskStatus::Success;
                    }
                }
                // println!("Continuing to move...");
                TaskStatus::Continue
            })
        .end()
    .end();
    let herbivore_behaviour = builder.build();
    let mut behaviour_map = BehaviourMap::default();
    behaviour_map.behaviours.insert("Herbivore".to_string(), herbivore_behaviour);
    resources.insert_thread_local(behaviour_map);

    let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();

    let materials = Materials {
        carn_mat: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        herb_mat: materials.add(Color::rgb(0.0, 0.75, 0.2).into()),
    };

    println!("Spawning herbivore...");

    spawn_herbivore(
        world,
        materials.herb_mat.clone(),
        [0.0, 0.0],
        20.0
    );

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
    mut herb_query: Query<(&Herbivore, &Transform)>,
    mut carn_query: Query<(&Carnivore, &Transform)>
) {
    for (mut ctx, mut trans) in ctx_query.iter_mut() {
        // for (_, other_trans) in other_herb_query.iter() {

        // }
    }
}

fn movement_system(
    // time resource
    time: Res<Time>,
    mut move_query: Query<(&Movement, &mut Transform, &mut CreatureContext)>
) {
    // interpolate towards the move target if not equal to it
    for (movement, mut trans, mut ctx) in move_query.iter_mut() {
        if let Some(ref target) = ctx.move_target {
            // get the difference between two points and give the new point, which is
            // in the direction of the 
            let level = movement.0 as f32;
            ctx.current_pos = move_point_towards(&ctx.current_pos, target, time.delta_seconds() * level);
            trans.translation.x = ctx.current_pos[0];
            trans.translation.y = ctx.current_pos[1];
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
struct Movement(u32);
struct DebugCircle;
struct BehaviourName(String);

fn spawn_herbivore(
    world: &mut World,
    material: Handle<ColorMaterial>,
    pos: Position,
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
        Movement(1),
        CreatureContext::default(),
        Planner::<CreatureContext>::default(),
        BehaviourName("Herbivore".to_owned()),
    )).unwrap();
}

const EPSILON: f32 = 0.001;
const MAX_MOVE_DISTANCE: f32 = 100.0;
const BOUNDARY: f32 = 400.0;
const MOVE_MULTIPLIER: f32 = 100.0;

fn locations_close(a: &Position, b: &Position) -> bool {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    (dx.powi(2) + dy.powi(2)).sqrt() <= EPSILON
}

fn random_nearby_location(p: &Position) -> Position {
    let mut rng = rand::thread_rng();
    let mid = MAX_MOVE_DISTANCE / 2.0;
    let dx: f32 = clamp_abs((MAX_MOVE_DISTANCE * rng.gen::<f32>()) - mid, BOUNDARY);
    let dy: f32 = clamp_abs((MAX_MOVE_DISTANCE * rng.gen::<f32>()) - mid, BOUNDARY);
    [
        p[0] + dx,
        p[1] + dy
    ]
}

// fn interpolate_positions(a: &Position, b: &Position, amount: f32) -> Position {
//     let mult = 100.0;
//     let c = [b[0] - a[0], b[1] - a[1]];
//     [
//         c[0] * amount * mult,q
//         c[1] * amount * mult
//     ]
// }

fn move_point_towards(from: &Position, to: &Position, amount: f32) -> Position {
    let diff = [to[0] - from[0], to[1] - from[1]];
    let diff_n = normalise(&[to[0] - from[0], from[1] - to[1]]);
    let move_to = [
        diff_n[0] * amount * MOVE_MULTIPLIER,
        diff_n[1] * amount * MOVE_MULTIPLIER,
    ];
    if length(&move_to) > length(&diff) {
        return to.clone();
    }
    move_to
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

fn normalise(a: &Position) -> Position {
    let length = (a[0].powi(2) + a[1].powi(2)).sqrt();
    [
        a[0] / length,
        a[1] / length
    ]
}

fn length(v: &[f32; 2]) -> f32 {
    (v[0].powi(2) + v[1].powi(2)).sqrt()
}