use bevy::prelude::*;
use crate::{
    input::MouseState,
    ui::*,
};

mod aspects;
pub use aspects::*;
mod commands;
pub use commands::*;

pub struct GamePlugin;

fn test_command_aspects(
    query: Query<(&Body, &Mind)>,
) {
    for (body, mind) in query.iter() {
        println!("Mind and body found as {:?},\n {:?}", mind, body);
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .register_type::<Body>()
            .register_type::<Mind>()
            .register_type::<Spirit>()
            .register_type::<Appearance>()
            .insert_resource(GameCommandQueue(vec![]))
            .add_system(execute_game_commands.exclusive_system())
            .add_system(appearance_added.system())
            .add_system(test_command_aspects.system())
        ;
    }
}

// UTILITY FNS

pub fn spawn_standard_boi(pos: &Vec2, cmds: &mut Commands, mouse: &MouseState) {
    let mut entity = cmds.spawn();
    let handler_entity = entity.id().clone();
    entity
        .insert(Appearance {
            filename: "textures/sprites/circle_lmao.png".to_string(),
            ..Default::default()
        })
        .insert(Body {
            strength: 10,
            coordination: 10,
            endurance: 10,
        })
        .insert(Transform::from_translation(pos.extend(100.0)))
        .insert(ClickHandlers {
            right: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
                cmds.spawn().insert(ContextMenuSpawn {
                    pos: mouse.ui_pos,
                    items: vec![
                        ContextMenuItem {
                            label: "Test".to_owned(),
                            handlers: Some(ClickHandlers {
                                left: Some(Box::new(move |cmds: &mut Commands, mouse: &MouseState| {
                                    println!("Durr!");
                                })),
                                ..Default::default()
                            }),
                        }
                    ]
                });
            })),
            ..Default::default()
        })
        ;
}