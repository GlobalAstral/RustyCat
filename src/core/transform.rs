use std::error::Error;

use macroquad::math::Rect;
use mlua::{Lua, Table, Value};

use crate::core::{core::Luable, vec2::Vec2};

pub struct Transform {
  pub pos: Vec2,
  pub size: Vec2,
  pub scale: f32,
}

impl Transform {
  pub fn new(pos: Vec2, size: Vec2) -> Transform {
    Transform { pos, size, scale: 1.0 }
  }

  pub fn instersects(&self, transform: &Transform) -> bool {
    let actual_size = self.size * self.scale;
    let a = Rect::new(self.pos.get_x() as f32, self.pos.get_y() as f32, actual_size.get_x() as f32, actual_size.get_y() as f32);
    let actual_size = transform.size * transform.scale;
    let b = Rect::new(transform.pos.get_x() as f32, transform.pos.get_y() as f32, actual_size.get_x() as f32, actual_size.get_y() as f32);
    a.overlaps(&b)
  }
}

impl Luable for Transform {
  fn as_lua(&mut self, lua: &Lua) -> Result<Value, Box<dyn Error>> {
    let pos: Value = self.pos.as_lua(lua)?;    
    let size: Value = self.size.as_lua(lua)?;
    let table: Table = lua.create_table()?;
    table.set("pos", pos)?;
    table.set("size", size)?;
    table.set("scale", Value::Number(self.scale as f64))?;
    Ok(Value::Table(table))
  }

  fn from_lua(&mut self, value: Value) -> Result<(), Box<dyn Error>> {
    if let Some(tbl) = value.as_table() {
      let pos: Value = tbl.get("pos")?;
      self.pos.from_lua(pos)?;
      let size: Value = tbl.get("size")?;
      self.size.from_lua(size)?;
      let scale: f32 = tbl.get("scale")?;
      self.scale = scale;
      return Ok(())
    }
    Err("Invalid Lua Value".into())
  }
}