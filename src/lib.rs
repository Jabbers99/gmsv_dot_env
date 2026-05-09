#![feature(c_unwind)]
use std::fs;
use dotenv_parser::parse_dotenv;

#[macro_use]
extern crate gmod;

unsafe extern "C-unwind" fn read_env(lua: gmod::lua::State) -> i32 {
    let contents = match fs::read_to_string("./garrysmod/.env") {
        Ok(c) => c,
        Err(_) => {
            eprintln!("./garrysmod/.env does not seem to exist");
            lua.push_nil();
            return 1;
        }
    };

    let env_vars = match parse_dotenv(&contents) {
        Ok(vars) => vars,
        Err(_) => {
            lua.push_nil();
            return 1;
        }
    };

    let lookup_var = lua.check_string(1).into_owned();

    if env_vars.contains_key(&lookup_var) {
        let desired_var = &env_vars[&lookup_var];
        lua.push_string(desired_var);
    } else {
        lua.push_nil();
    }

    return 1;
}

#[gmod13_open]
pub unsafe extern "C-unwind" fn gmod13_open(lua: gmod::lua::State) -> i32 {
    lua.push_function(read_env);
    lua.set_global(lua_string!("env"));

    lua.pop();

    return 0;
}

#[gmod13_close]
fn gmod13_close(_lua: gmod::lua::State) -> i32 {
    return 0;
}
