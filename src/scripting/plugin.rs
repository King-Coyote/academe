use bevy::prelude::*;
use rlua::prelude::*;
use crate::scripting::*;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_resource(Scripting::new())
        .add_startup_system(startup.system())
        // .add_system(added_script.system())
        .add_system(script.system());
    }
}

fn startup(
    mut commands: Commands,
    scripting: Res<Scripting>,
) {
    println!("Initialising scripting system.");
    // commands.spawn((
    //     Script {
    //         table_key: None,
    //         filename: "test_script.lua".to_owned()
    //     }, 
    // ));
}