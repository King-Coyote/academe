use crate::scripting::*;
use bevy::prelude::*;
use rlua::{RegistryKey, Table, Result, Function, Context, Error};
use rlua::prelude::*;
use std::ops::Deref;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct Script {
    pub table_key: Option<RegistryKey>,
    pub filename: String,
}

pub fn script(
    mut scripting: ResMut<Scripting>,
    mut script_query: Query<(&mut Script)>
) {
    scripting.get().context(|ctx| -> Result<(())> {
        for mut script in &mut script_query.iter() {
            match &script.table_key {
                Some(key) => {
                    let script_table: Table = ctx.registry_value(key)?;
                    call_if_exists(&script_table, "update")?;
                },
                None => {
                    let table: Table = eval_lua_file(&ctx, &script.filename)?;
                    script.table_key = Some(ctx.create_registry_value(table.clone())?);
                    call_if_exists(&table, "added")?;
                }
            }
        }
        Ok(())
    }).map_err(|e| {
        println!("Script component error: {}",e );
    }).ok();
}

// pub fn added_script(
//     mut scripting: ResMut<Scripting>,
//     mut added_query: Query<(Added<Script>, &mut Script)>
// ) {
//     scripting.get().context(|ctx| -> Result<(())> {
//         for (_, mut script) in &mut added_query.iter() {
//             // let table = ctx.load(&script.filename).eval::<Table>()?;
//             // script.table_key = Some(ctx.create_registry_value(table)?);
//             // println!("Successfully loaded script at {}", script.filename)
//             println!("Running for added script component");
//         }
//         Ok(())
//     }).map_err(|e| {
//         println!("Error adding script component: {}",e );
//     }).ok();
// }

// utility fns
pub fn eval_lua_file<'lua, P, R>(ctx: &Context<'lua>, path: P) -> Result<R> 
    where 
        P: AsRef<Path>,
        R: FromLuaMulti<'lua>
{
    let full_path = get_asset_path("scripts")
        .join(path);
    match fs::read_to_string(full_path) {
        Ok(contents) => {
            let parsed = contents.parse::<String>().unwrap();
            let val: R = ctx.load(&parsed).eval()?;
            Ok(val)
        },
        Err(err) => return Err(Error::ExternalError(Arc::new(err)))
    }
}

// do eval but don't worry about return value
pub fn exec_lua_file<'lua, P>(ctx: &Context<'lua>, path: P) -> Result<()> 
    where 
        P: AsRef<Path>,
{
    eval_lua_file::<_, ()>(ctx, path)?;
    Ok(())
}

pub fn get_asset_path<P: AsRef<Path>>(p: P) -> PathBuf {
    env::current_dir().unwrap()
        .join("assets")
        .join(p)
}

// call a function safely on a table, if it exists, returning any errors
fn call_if_exists<'lua>(t: &Table, name: &str) -> Result<()> {
    if let Ok(f) = t.get::<_, Function>(name) {
        f.call(())?;
    }
    Ok(())
}