use crate::{
    input::MouseState,
    ui::*,
    nav::{NavAgent, NavAgentStrategy},
    ai::{ActorStore, EnemyContext},
    game::*,
};
use bevy::{prelude::*, text::Text2dSize};
use bevy_htn::prelude::*;
use bevy_prototype_lyon::prelude::*;

// UTILITY FNS
pub fn spawn_standard_boi(
    pos: Vec2,
    mouse: &MouseState,
    text_style: TextStyle,
    cmds: &mut Commands,
    is_enemy: bool,
) {
    info!("Spawning student at {}", pos);
    let mut entity = cmds.spawn();
    let spawned = entity.id();
    let filename = {
        if is_enemy {
            "textures/sprites/enemy.png".to_string()
        } else {
            "textures/sprites/player.png".to_string()
        }
    };
    entity
        .insert(Appearance {
            filename,
            ..Default::default()
        })
        .insert(Body {
            strength: 10,
            coordination: 10,
            endurance: 10,
        })
        .insert(Movement{level: 1})
        .insert(NavAgent::with_strategy(NavAgentStrategy::FreeRoam))
        .insert(Transform::from_translation(pos.extend(100.0)))
        .insert(ObjectInteraction::default())
        .insert(ClickHandlers {
            right: context_menu_handler!(cmds, mouse, {
                label: "Annihilate",
                action: {
                    cmds.entity(spawned).despawn_recursive();
                }
            }),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    "Entity",
                    text_style
                ),
                text_2d_size: Text2dSize{size: Vec2::new(100.0, 20.0)},
                transform: Transform::from_xyz(-25.0, 60.0, 0.0),
                ..default()
            });
        });
        
    if is_enemy {
        entity.insert(Enemy)
            .insert(EnemyContext {
                name: "BeEnemy".to_string(),
                state: ContextState::default(),
                actor_store: ActorStore {
                    move_target: None,
                    current_pos: pos,
                    wants_new_location: true,
                    current_time: 0.0,
                }
            })
            // TODO UPDATE
            // .insert(Planner::<EnemyContext>::default())
            ;
    }
}

#[derive(Component)]
pub struct Room;

pub fn spawn_room(
    commands: &mut Commands,
    name: &str,
    pos: Vec2,
) -> Entity {
    let shape = GeometryBuilder::build_as(
        &shapes::Rectangle {
            extents: Vec2::new(100.0, 100.0),
            origin: RectangleOrigin::Center,
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLUE, 2.0),
        },
        Transform::from_xyz(pos.x, pos.y, 0.0),
    );
    // let shape = shapes::RegularPolygon {
    //     sides: 6,
    //     feature: shapes::RegularPolygonFeature::Radius(200.0),
    //     ..shapes::RegularPolygon::default()
    // };
    info!("spawning room at {:?}", pos);
    let mut builder = commands.spawn();
    let entity = builder.id();
    builder
        .insert(Room)
        .insert_bundle(shape);
    entity
}