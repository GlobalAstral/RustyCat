
use std::{error::Error, path::PathBuf, str::FromStr};

use macroquad::{prelude::warn, time::get_frame_time, window::{clear_background, next_frame}};
use mlua::{Lua, MultiValue, Table, Value};

use crate::core::{children_container::ChildrenContainer, color::Color, core::Luable, nodelike::NodeLike};

pub struct Engine {
  pub bg_color: Color,
  pub children: ChildrenContainer<String, Box<dyn NodeLike>>,
  
  update: Option<Box<dyn FnMut(&mut Engine, f32)>>,
  setup: Option<Box<dyn FnMut(&mut Engine)>>,

  lua: Lua,
}

impl Engine {
  pub fn new(bg: Color) -> Self {
    Engine {
      bg_color: bg,
      children: ChildrenContainer::new(),
      lua: Lua::new(),
      setup: None,
      update: None
    }
  }

  pub fn setSetup(&mut self, f: Box<dyn FnMut(&mut Engine)>) {
    self.setup = Some(f);
  }

  pub fn setUpdate(&mut self, f: Box<dyn FnMut(&mut Engine, f32)>) {
    self.update = Some(f);
  }

  pub fn add_script_to_node<N>(&self, node: &mut N, path: &str) where N: NodeLike + Luable {
    let this: Value = node.as_lua(&self.lua).expect("Cannot get lua value of node");
    node.get_scripts().addScript(PathBuf::from_str(path).expect("Invalid Path"), &self.lua, this).expect("Cannot add script to node");
  }

  pub async fn mainloop(&mut self) {
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
    let temp_setup = std::mem::take(&mut self.setup);
    if let Some(mut setup) = temp_setup {
      setup(self);
      self.setup = Some(setup);
    } else {
      warn!("Setup Engine Handler is not set");
      self.setup = temp_setup;
    }
    loop {
      let dt: f32 = get_frame_time();
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
      let temp_update = std::mem::take(&mut self.update);
      if let Some(mut update) = temp_update {
        update(self, dt);
        self.update = Some(update);
      } else {
        warn!("Update Engine Handler is not set");
        self.update = temp_update;
        self.update = Some(Box::new(|_, _| { }))
      }

      clear_background(self.bg_color.into());
      self.children.foreach_child(|_, _ , child| {
        child.render();
      });
      next_frame().await;
    }
  }
}
