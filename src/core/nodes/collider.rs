
use std::sync::{Arc, Mutex};

use mlua::{Function, Table, Value};
use once_cell::sync::Lazy;

use crate::core::{core::{Downcastable, Luable}, nodelike::NodeLike, nodes::node::Node, transform::Transform, vec2::Vec2};

static COLLIDER_MANAGER: Lazy<Mutex<Vec<Arc<Collider>>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub struct Collider {
  base: Node,
  transform: Transform,
  layer: String,
}

impl Collider {
  pub fn new(pos: Vec2, size: Vec2, layer: String) -> Arc<Collider> {
    let ret = Arc::new(Collider { base: Node::new(), transform: Transform::new(pos, size), layer: layer });
    COLLIDER_MANAGER.lock().unwrap().push(ret.clone());
    ret
  }

  pub fn empty() -> Collider {
    Collider { base: Node::new(), transform: Transform::new(Vec2::ZERO, Vec2::ZERO), layer: String::new() }
  }
}

impl NodeLike for Collider {
  fn get_kind(&self) -> &str {
    "Collider"
  }
  fn get_scripts(&mut self) -> &mut crate::core::script_manager::ScriptManager {
    self.base.get_scripts()
  }
  fn load_scripts(&mut self) {
    let tmp = self.get_kind().to_string();
    self.base.load_scripts(&tmp);
  }
  fn render(&mut self) {
    self.base.render();
  }
  fn setup(&mut self) {
    self.base.setup();
  }
  fn update(&mut self, deltatime: f32) {
    self.base.update(deltatime);
  }
}

impl Downcastable for Collider {
  fn as_any(&mut self) -> &mut dyn std::any::Any {
    self
  }
}

impl Luable for Collider {
  fn as_lua(&self, lua: &mlua::Lua) -> Result<mlua::Value, Box<dyn std::error::Error>> {
    let table = lua.create_table()?;
    table.set("base", self.base.as_lua(lua)?)?;
    table.set("transform", self.transform.as_lua(lua)?)?;
    table.set("layer", self.layer.clone())?;
    table.set("collides", lua.create_function(|_, (this, force_all): (Table, Option<bool>)| {
      let mut transform: Transform = Transform::new(Vec2::ZERO, Vec2::ZERO);
      transform.from_lua(this.get("transform")?).expect("Invalid Lua Value");
      let layer: String = this.get("layer")?;
      let id: u64 = this.get::<Table>("base")?.get::<Function>("id")?.call::<u64>(())?;
      let manager = COLLIDER_MANAGER.lock().unwrap();
      let mut to_check = manager.iter().filter(|coll| coll.base.id != id && coll.layer == layer);
      let flag = if force_all.is_some() && force_all.unwrap() {
        to_check.all(|ele| transform.instersects(&ele.transform))
      } else {
        to_check.any(|ele| transform.instersects(&ele.transform))
      };
      Ok(flag)
    })?)?;
    Node::add_kind_to_lua(self.get_kind().to_string(), &table, lua)?;
    Ok(Value::Table(table))
  }

  fn from_lua(&mut self, value: Value) -> Result<(), Box<dyn std::error::Error>> {
    let table: &Table = value.as_table().ok_or("Invalid Lua Value")?;
    self.base.from_lua(table.get("base")?)?;
    self.transform.from_lua(table.get("transform")?)?;
    self.layer = table.get("layer")?;
    Ok(())
  }
}
