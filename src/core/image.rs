use futures::executor::block_on;
use macroquad::{math::Rect, texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture}};
use mlua::Value;

use crate::core::{color::Color, core::{Luable, radians}, vec2::Vec2};


pub struct Img {
  texture: String,
  rotation: f32,
  src: Option<Vec2>,
  tint: Color,
  flip_x: bool,
  flip_y: bool,
}

impl Img {
  pub fn new(path: &str) -> Img {
    Img { 
      texture: path.to_string(), 
      rotation: 0.0, 
      src: None,
      tint: Color::new(0xffffffff),
      flip_x: false,
      flip_y: false
    }
  }

  pub fn with_degrees(mut self, degrees: f32) -> Self {
    self.rotation = radians(degrees);
    self
  }
  pub fn with_radians(mut self, radians: f32) -> Self {
    self.rotation = radians;
    self
  }
  pub fn section(mut self, pos: Vec2) -> Self {
    self.src = Some(pos);
    self
  }
  pub fn flip(mut self, x: bool, y: bool) -> Self {
    self.flip_x = x;
    self.flip_y = y;
    self
  }
  pub fn tint(mut self, col: Color) -> Self {
    self.tint = col;
    self
  }
  pub fn render(&self, pos: Vec2, size: Vec2) {
    let tex: Texture2D = block_on(load_texture(&self.texture)).expect(&format!("Cannot load image '{}'", self.texture));
    draw_texture_ex(
      &tex, 
      pos.get_x() as f32, 
      pos.get_y() as f32, 
      self.tint.into(), 
      DrawTextureParams {
        dest_size: Some(macroquad::math::Vec2 { x: size.get_x() as f32, y: size.get_y() as f32 }),
        rotation: self.rotation,
        source: match self.src {
          Some(v) => Some(Rect::new(v.get_x() as f32, v.get_y() as f32, size.get_x() as f32, size.get_y() as f32)),
          None => None
        },
        flip_x: self.flip_x,
        flip_y: self.flip_y,
        pivot: None
      }
    );
  }
}

impl Luable for Img {
  fn as_lua(&mut self, lua: &mlua::Lua) -> Result<mlua::Value, Box<dyn std::error::Error>> {
    let table: mlua::Table = lua.create_table()?;
    table.set("texture", self.texture.clone())?;
    table.set("rotation", self.rotation)?;
    table.set("src", match self.src {
      Some(mut vec) => vec.as_lua(lua)?,
      None => Value::Nil
    })?;
    table.set("tint", self.tint.as_lua(lua)?)?;
    table.set("flip_x", self.flip_x)?;
    table.set("flip_y", self.flip_y)?;
    Ok(Value::Table(table))
  }

  fn from_lua(&mut self, value: Value) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(table) = value.as_table() {
      self.texture = table.get::<String>("texture")?;
      self.rotation = table.get("rotation")?;
      self.src = match table.get::<Value>("src")? {
        Value::Table(tbl) => {
          let mut tmp = Vec2::new(0, 0);
          tmp.from_lua(Value::Table(tbl))?;
          Some(tmp)
        },
        _ => None
      };
      self.tint.from_lua(table.get("tint")?)?;
      self.flip_x = table.get("flip_x")?;
      self.flip_y = table.get("flip_y")?;
      return Ok(())
    }
    Err("Invalid Lua Value".into())
  }
}
