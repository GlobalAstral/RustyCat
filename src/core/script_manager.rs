use std::{error::Error, fs, path::PathBuf};

use macroquad::{input::{KeyCode, MouseButton, is_key_down, is_key_pressed, is_key_released, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position}, window::{screen_height, screen_width}};
use mlua::{Chunk, Function, Lua, MultiValue, Table, Value};

use crate::core::{core::{Luable, init_env_commons}, keys::Stringable, vec2::Vec2};

pub struct ScriptManager {
  scripts: Vec<(String, Chunk<'static>, Table)>,
  environments: Option<Vec<Table>>
}

impl ScriptManager {
  pub fn new() -> ScriptManager { 
    ScriptManager {
      scripts: Vec::new(),
      environments: None
    }
  }

  pub fn stringify(ele: &Value, depth: usize) -> String {
    match ele {
      Value::Boolean(b) => b.to_string(),
      Value::Integer(i) => i.to_string(),
      Value::Error(e) => e.to_string(),
      Value::Nil => "nil".to_string(),
      Value::Number(f) => f.to_string(),
      Value::String(st) => st.to_string_lossy().to_string(),
      Value::LightUserData(_) => "<lightuserdata>".to_string(),
      Value::Other(_) => "<other>".to_string(),
      Value::Thread(_) => "<thread>".to_string(),
      Value::UserData(_) => "<userdata>".to_string(),
      Value::Function(f) => format!("<function>({})", match f.info().name {
        Some(n) => n,
        _ => "anonymous".to_string()
      }),
      Value::Table(t) => {
        let mut ret: String = String::new();

        ret.push_str("{\n");
        t.for_each(|k: Value, v: Value| {
          let key: String = ScriptManager::stringify(&k, depth + 1);
          let value: String = ScriptManager::stringify(&v, depth + 1);
          let r = format!("\t{}{}: {}\n", "\t".repeat(depth), key, value);
          ret.push_str(&r);
          Ok(())
        }).expect("Cannot print Table");
        ret.push_str(&format!("{}}}", "\t".repeat(depth)));
        ret
      },
    }
  }

  fn load_persistrent(&self, lua: &Lua, env: &Table) -> Result<(), Box<dyn Error>> {
    env.set("window_width", screen_width())?;
    env.set("window_height", screen_height())?;
    let mut tmp: Vec2 = {
      let (mx, my) = mouse_position();
      Vec2::new(mx as i32, my as i32)
    };
    env.set("mouse_pos", tmp.as_lua(lua)?)?;
    Ok(())
  }

  fn create_environment(&self, lua: &Lua, this: Value) -> Result<Table, Box<dyn Error>> {
    let env: Table = lua.create_table()?;
    env.set("this", this)?;
    self.load_persistrent(lua, &env)?;

    init_env_commons(lua, &env)?;

    Ok(env)
  }

  pub fn addScript(&mut self, path: PathBuf, lua: &Lua, this: Value) -> Result<(), Box<dyn Error>> {
    let tmp: PathBuf = path.clone();
    let filename: &str = tmp.file_name().ok_or_else(|| "Path has no filename")?.to_str().unwrap();
    let src: String = fs::read_to_string(path)?;
    let fname: String = filename.to_string();
    let chunk: Chunk<'_> = lua.load(src);
    let env = self.create_environment(lua, this)?;
    self.scripts.push((fname, chunk, env));
    Ok(())
  }

  pub fn loadScripts(&mut self) -> Result<(), Box<dyn Error>> {
    let mut ret: Vec<Table> = Vec::new();
    let tmp = std::mem::take(&mut self.scripts); //? WILL NOT BE GIVEN BACK TO THE MANAGER.
    for (fname, chunk, env) in tmp {
      chunk
        .set_environment(env.clone())
        .set_name(fname.clone())
        .exec()?;
      ret.push(env);
    }
    self.environments = Some(ret);
    Ok(())
  }

  pub fn run_4all_envs(&mut self, lua: &Lua, func_name: String, args: MultiValue) -> Result<Option<Table>, Box<dyn Error>> {
    if self.environments.is_none() {
      return Ok(None);
    }
    let envs: &Vec<Table> = self.environments.as_ref().unwrap();
    for env in envs {
      self.load_persistrent(lua, env)?;
      let func: Function = env.get(func_name.clone())?;
      let _: Value = func.call(args.clone())?;
    }

    let ret = envs.last();
    if ret.is_none() {
      return Ok(None)
    }
    let ret = ret.unwrap();
    let tbl: Table = ret.get("this")?;
    Ok(Some(tbl))
  }
}