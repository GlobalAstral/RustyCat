
use std::{error::Error, fs, path::PathBuf, process::Child, str::FromStr, sync::{Arc, RwLock, RwLockReadGuard}};

use lazy_static::lazy_static;
use macroquad::{input::{KeyCode, MouseButton, is_key_down, is_key_pressed, is_key_released, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released}, prelude::warn, time::get_frame_time, window::{clear_background, next_frame}};
use mlua::{Chunk, ExternalError, Function, Lua, MultiValue, Table, Value};

use crate::core::{children_container::ChildrenContainer, color::Color, core::{Downcastable, Luable, call_constructor, init_env_commons, load_persistrent}, image::Img, keys::Stringable, nodelike::NodeLike, nodes::{camera::Camera, clickable_area::ClickableArea, node::Node, rectmesh::RectMesh, sprite::Sprite}, script_manager::ScriptManager, vec2::Vec2};

lazy_static! {
  pub static ref MAIN_CAMERA: Arc<RwLock<Option<Camera>>> = Arc::new(RwLock::new(None));
}

pub fn main_camera<'a>() -> RwLockReadGuard<'a, Option<Camera>> {
  MAIN_CAMERA.read().unwrap()
}

pub struct Engine {
  pub bg_color: Color,
  pub children: ChildrenContainer<String, Box<dyn NodeLike + Send + Sync>>,
  
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

  fn load_children(&mut self) {
    let new_children: Table = self.environment.get("root").expect("Cannot update properties of 'root'");
      self.children.clear_children();
      new_children.for_each(|name: String, node: Table| {
        let kind: String = node.get::<Function>("kind")?.call::<String>(())?;
        let gotten_node : Box<dyn NodeLike + Send + Sync> = call_constructor(&kind, Value::Table(node))?;
        self.children.add_child(name, gotten_node);
        Ok(())
      }).expect("Cannot iterate root children");
  }

  fn init_env(lua: &Lua, env: &Table) -> Result<(), Box<dyn Error>> {
    env.set("root", Value::Table(lua.create_table()?))?;
    
    init_env_commons(lua, env)?;

    let environment = env.clone();
    env.set("add_node", lua.create_function_mut(move |this, (name, node): (String, Table)| {
      let later = node.clone();
      
      let root: Table = environment.get("root")?;
      root.set(name, node)?;

      Ok(later)
    })?)?;

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
    load_persistrent(&self.lua, &self.environment).expect("Cannot load Persistent Data");
    if let Ok(func) = self.environment.get::<Function>("Setup") {
      func.call::<()>(()).expect("Error during Engine Setup");
    } else {
      warn!("No Engine Setup function");
    }

    self.children.foreach_child(|_, _ , child| {
      child.load_scripts();
    });

    self.load_children();

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

      load_persistrent(&self.lua, &self.environment).expect("Cannot load Persistent Data");
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
            eprintln!("ERROR: {}", tmp.err().unwrap());
          } else {
            let tmp: Option<Table> = tmp.unwrap();
            if let Some(this) = tmp {
              child.from_lua(Value::Table(this)).expect("Cannot update properties of 'this'");
            }
          }
        });
      self.lua = lua_temp;

      self.load_children();

      clear_background(self.bg_color.into());
      self.children.foreach_child(|_, _ , child| {
        child.render();
      });
      next_frame().await;
    }
  }
}
