use bevy::prelude::*;
use bevy_htn::prelude::*;
use rand::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugin(ShapePlugin)
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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let materials = Materials {
        carn_mat: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        herb_mat: materials.add(Color::rgb(0.0, 0.75, 0.2).into()),
    };

    println!("Spawning herbivore...");

    spawn_herbivore(
        commands,
        materials.herb_mat.clone(),
        [0.0, 0.0],
        20.0
    );

    commands.insert_resource(materials);

    commands.spawn(Camera2dBundle::default());

    // let mut builder: BehaviourBuilder<HerbivoreContext> = BehaviourBuilder::new("Herbivore");
    // use Variant::*;
    // builder
    // .selector("BeAHerbivore")
    //     .primitive("MoveRandomly")
    //         .condition("The timer has expired", |ctx: &HerbivoreContext| ctx.move_timer_expired )
    //         .do_action("Choose new location", |ctx: &mut HerbivoreContext| -> TaskStatus {
    //             if locations_close(&ctx.move_target, &ctx.current_pos) {
    //                 // make a new move loc
    //                 ctx.move_target = random_nearby_location(&ctx.move_target);
    //                 return TaskStatus::Success;
    //             }
    //             TaskStatus::Continue
    //         })
    //     .end()
    // .end()
    // ;
    // let behaviour = builder.build();
    // behaviour.print();
    // let mut planner = Planner::default();
    // let mut ctx = HerbivoreContext::default();
    // planner.tick(&behaviour, &mut ctx);

}

struct Materials {
    carn_mat: Handle<ColorMaterial>,
    herb_mat: Handle<ColorMaterial>,
}

struct Herbivore;
struct Movement(u32);
struct DebugCircle;

fn spawn_herbivore(
    mut commands: &mut Commands,
    material: Handle<ColorMaterial>,
    pos: [f32; 2],
    r: f32,
) {
    let circle = shapes::Circle {
        radius: r,
        ..Default::default()
    };

    commands.spawn(
        circle.draw(
            material,
            TessellationMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(pos[0], pos[1], 0.0)),
        )
    )
    .with(DebugCircle)
    .with(Herbivore)
    .with(Movement(1))
    .with(HerbivoreContext::default())
    .with(Planner::<HerbivoreContext>::default())
    ;
}

const EPSILON: f32 = 0.001;

fn locations_close(a: &[f32; 2], b: &[f32; 2]) -> bool {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    (dx.powi(2) + dy.powi(2)).sqrt() <= EPSILON
}

const MAX_MOVE_DISTANCE: f32 = 300.0;
const BOUNDARY: f32 = 400.0;

fn random_nearby_location(p: &[f32; 2]) -> [f32; 2] {
    let mut rng = rand::thread_rng();
    let mid = MAX_MOVE_DISTANCE / 2.0;
    let dx: f32 = clamp_abs((MAX_MOVE_DISTANCE * rng.gen::<f32>()) - mid, BOUNDARY);
    let dy: f32 = clamp_abs((MAX_MOVE_DISTANCE * rng.gen::<f32>()) - mid, BOUNDARY);
    [
        p[0] + dx,
        p[1] + dy
    ]
}

fn clamp_abs(mut a: f32, to: f32) -> f32 {
    if a > to {a = to;}
    if a < to * -1.0 {a = to * -1.0;}
    a
}