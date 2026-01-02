use std::{any::Any, error::Error};

use macroquad::shapes::draw_rectangle;
use mlua::{Lua, Value};

use crate::core::{color::Color, core::{Downcastable, Luable}, nodelike::NodeLike, nodes::node::Node, script_manager::ScriptManager, transform::Transform, vec2::Vec2};

pub struct RectMesh {
  base: Node,
  pub transform: Transform,
  pub color: Color
}

impl RectMesh {
  pub fn new(pos: Vec2, size: Vec2, color: Color) -> RectMesh {
    RectMesh { base: Node::new(), transform: Transform::new(pos, size), color: color }
  }
}

impl NodeLike for RectMesh {
  fn render(&mut self) {
    self.base.render();
    draw_rectangle(self.transform.pos.get_x() as f32, self.transform.pos.get_y() as f32, self.transform.size.get_x() as f32 * self.transform.scale, self.transform.size.get_y() as f32 * self.transform.scale, self.color.into());
  }
  fn setup(&mut self) {
    self.base.setup();
  }
  fn update(&mut self, deltatime: f32) {
    self.base.update(deltatime);
  }
  fn load_scripts(&mut self) {
    let tmp = self.get_kind().to_string();
    self.base.load_scripts(&tmp);
  }
  fn get_scripts(&mut self) -> &mut ScriptManager {
    self.base.get_scripts()
  }
  fn get_kind(&self) -> &str {
    "RectMesh"
  }
}

impl Downcastable for RectMesh {
  fn as_any(&mut self) -> &mut dyn Any {
    self
  }
}

impl Luable for RectMesh {
  fn as_lua(&mut self, lua: &Lua) -> Result<Value, Box<dyn Error>> {
    let table = lua.create_table()?;
    let base: Value = self.base.as_lua(lua)?;
    table.set("base", base)?;

    let transform = self.transform.as_lua(lua)?;
    table.set("transform", transform)?;

    let color = self.color.as_lua(lua)?;
    table.set("color", color)?;
    Node::add_kind_to_lua(self.get_kind().to_string(), &table, lua)?;

    Ok(Value::Table(table))
  }

  fn from_lua(&mut self, value: Value) -> Result<(), Box<dyn Error>> {
    if let Some(tbl) = value.as_table() {
      let base: Value = tbl.get("base")?;
      self.base.from_lua(base)?;
      let transform: Value = tbl.get("transform")?;
      self.transform.from_lua(transform)?;
      let color: Value = tbl.get("color")?;
      self.color.from_lua(color)?;
      return Ok(())
    }

    Err("Invalid Lua Value".into())
  }
}
