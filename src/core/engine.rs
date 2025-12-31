
use std::{error::Error, fs, path::PathBuf, str::FromStr};

use macroquad::{input::{KeyCode, MouseButton, is_key_down, is_key_pressed, is_key_released, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released}, prelude::warn, time::get_frame_time, window::{clear_background, next_frame}};
use mlua::{Chunk, Function, Lua, MultiValue, Table, Value};

use crate::core::{children_container::ChildrenContainer, color::Color, core::Luable, keys::Stringable, nodelike::NodeLike, script_manager::ScriptManager};

pub struct Engine {
  pub bg_color: Color,
  pub children: ChildrenContainer<String, Box<dyn NodeLike>>,
  
  environment: Table,

  lua: Lua,

}

impl Engine {
  pub fn new(bg: Color, env: Table) -> Self {
    Engine {
      bg_color: bg,
      children: ChildrenContainer::new(),
      lua: Lua::new(),
      environment: env
    }
  }

  fn init_env(lua: &Lua, env: &Table) -> Result<(), Box<dyn Error>> {
    env.set("print", lua.create_function(|_, mut args: MultiValue| {
      let mut default_sep = ", ".to_string();
      if let Some(Value::String(s)) = args.iter().last() {
        let s = s.to_str()?;
        if s.starts_with("sep=") {
          default_sep = s[4..].to_string();
          args.pop_back();
        }
      }
      let parts: Vec<String> = args.iter().map(|ele| {ScriptManager::stringify(ele, 0)}).collect();
      println!("{}", parts.join(&default_sep));
      Ok(())
    })?)?;

    env.set("keydown", lua.create_function(|_, key: String| {
      let code: Option<Box<KeyCode>> = KeyCode::from_string(&key);
      if let Some(keycode) = code {
        return Ok(is_key_down(*keycode))
      }
      Ok(false)
    })?)?;

    env.set("keypressed", lua.create_function(|_, key: String| {
      let code: Option<Box<KeyCode>> = KeyCode::from_string(&key);
      if let Some(keycode) = code {
        return Ok(is_key_pressed(*keycode))
      }
      Ok(false)
    })?)?;

    env.set("keyreleased", lua.create_function(|_, key: String| {
      let code: Option<Box<KeyCode>> = KeyCode::from_string(&key);
      if let Some(keycode) = code {
        return Ok(is_key_released(*keycode))
      }
      Ok(false)
    })?)?;

    env.set("mkeydown", lua.create_function(|_, key: i64| {
      let button = match key {
        0 => MouseButton::Left,
        1 => MouseButton::Right,
        2 => MouseButton::Middle,
        _ => {
          return Ok(false);
        }
      };
      Ok(is_mouse_button_down(button))
    })?)?;

    env.set("mkeypressed", lua.create_function(|_, key: i64| {
      let button = match key {
        0 => MouseButton::Left,
        1 => MouseButton::Right,
        2 => MouseButton::Middle,
        _ => {
          return Ok(false);
        }
      };
      Ok(is_mouse_button_pressed(button))
    })?)?;

    env.set("mkeyreleased", lua.create_function(|_, key: i64| {
      let button = match key {
        0 => MouseButton::Left,
        1 => MouseButton::Right,
        2 => MouseButton::Middle,
        _ => {
          return Ok(false);
        }
      };
      Ok(is_mouse_button_released(button))
    })?)?;

    //TODO do more stuff

    Ok(())
  } 

  pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
    let lua: Lua = Lua::new();
    let file_content: String = fs::read_to_string(path)?;
    let chunk: Chunk = lua.load(file_content);
    let environment: Table = lua.create_table()?;
    Engine::init_env(&lua, &environment)?;
    chunk.set_environment(environment.clone()).exec()?;

    let mut color: Color = Color::new(0);
    let tmp = environment.get::<Table>("color");
    if let Ok(tbl) = tmp {
      color.from_lua(Value::Table(tbl)).ok();
    }
    Ok( Engine { bg_color: color, children: ChildrenContainer::new(), environment, lua } )
  }

  pub fn add_script_to_node<N>(&self, node: &mut N, path: &str) where N: NodeLike + Luable {
    let this: Value = node.as_lua(&self.lua).expect("Cannot get lua value of node");
    node.get_scripts().addScript(PathBuf::from_str(path).expect("Invalid Path"), &self.lua, this).expect("Cannot add script to node");
  }

  pub async fn mainloop(&mut self) {
    if let Ok(func) = self.environment.get::<Function>("Setup") {
      func.call::<()>(()).expect("Error during Engine Setup");
    } else {
      warn!("No Engine Setup function");
    }

    self.children.foreach_child(|_, _ , child| {
      child.load_scripts();
    });
    self.children.foreach_child(|_, _ , child| {
        child.setup();
        let tmp= child.get_scripts().run_4all_envs(&self.lua, "Setup".into(), MultiValue::new());
        if tmp.is_err() {
          warn!("Error during setup in script");
          println!("ERROR: {}", tmp.err().unwrap());
        } else {
          let tmp: Option<Table> = tmp.unwrap();
          if let Some(this) = tmp {
            child.from_lua(Value::Table(this)).expect("Cannot update properties of 'this'");
          }
        }
      });
    loop {
      let dt: f32 = get_frame_time();

      if let Ok(func) = self.environment.get::<Function>("Loop") {
        func.call::<()>(dt).expect("Error during Engine Loop");
      } else {
        warn!("No Engine Loop function");
      }

      let tmp = dt;
      let lua_temp: Lua = std::mem::take(&mut self.lua);
      self.children.foreach_child(|_, _ , child| {
          child.update(tmp);
          
          let tmp: Result<Option<Table>, Box<dyn Error>>  = child.get_scripts().run_4all_envs(&lua_temp, "Loop".into(), MultiValue::from_vec(vec![Value::Number(dt as f64)]));
          if tmp.is_err() {
            warn!("Error during loop in script");
            println!("ERROR: {}", tmp.err().unwrap());
          } else {
            let tmp: Option<Table> = tmp.unwrap();
            if let Some(this) = tmp {
              child.from_lua(Value::Table(this)).expect("Cannot update properties of 'this'");
            }
          }
        });
      self.lua = lua_temp;

      clear_background(self.bg_color.into());
      self.children.foreach_child(|_, _ , child| {
        child.render();
      });
      next_frame().await;
    }
  }
}
