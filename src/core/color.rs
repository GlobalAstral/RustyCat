use std::error::Error;

use mlua::{Lua, Value};

use crate::core::core::Luable;

#[derive(Debug, Clone, Copy)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
  a: u8
}

impl Into<macroquad::color::Color> for Color {
  fn into(self) -> macroquad::color::Color {
    macroquad::color::Color { r: self.get_nr(), g: self.get_ng(), b: self.get_nb(), a: self.get_na() }
  }
}

impl Color {
  pub fn new(hex: u32) -> Self {
    let a: u32 = (hex & 0xFF000000) >> 24;
    let r: u32 = (hex & 0x00FF0000) >> 16; 
    let g: u32 = (hex & 0x0000FF00) >> 8;
    let b: u32 = (hex & 0x000000FF) >> 0;
    Color { r: r as u8, g: g as u8, b: b as u8, a: a as u8 }
  }
  pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
    Color { r, g, b, a }
  }

  pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
    Color::from_rgba(r, g, b, 255)
  }

  pub fn from_nrgba(r: f32, g: f32, b: f32, a: f32) -> Self {
    Color::from_rgba((r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8, (a*255.0) as u8)
  }

  pub fn from_nrgb(r: f32, g: f32, b: f32) -> Self {
    Color::from_nrgba(r, g, b, 1.0)
  }

  pub fn from_hex(hex: &str) -> Self {
    let mut col: String = hex.to_string();
    if col.len() > 8 {
      col.truncate(8);
    } else if col.len() < 8 {
      let to_fill = 8 - col.len();
      let tmp = &("0".repeat(to_fill));
      col.push_str(tmp);
    }

    let chunks: Vec<String> = col
      .chars()
      .collect::<Vec<char>>()
      .chunks(2)
      .map(|chunk| chunk.iter().collect())
      .collect();

    let a: u8 = u8::from_str_radix(&chunks[0], 16).expect("Invalid HEX alpha value");
    let r: u8 = u8::from_str_radix(&chunks[1], 16).expect("Invalid HEX red value");
    let g: u8 = u8::from_str_radix(&chunks[2], 16).expect("Invalid HEX green value");
    let b: u8 = u8::from_str_radix(&chunks[3], 16).expect("Invalid HEX blue value");

    Color { r, g, b, a }
  }

  pub fn get_r(&self) -> u8 {
    self.r
  }

  pub fn get_g(&self) -> u8 {
    self.g
  }

  pub fn get_b(&self) -> u8 {
    self.b
  }

  pub fn get_a(&self) -> u8 {
    self.a
  }

  pub fn get_nr(&self) -> f32 {
    (self.r / 255) as f32
  }

  pub fn get_ng(&self) -> f32 {
    (self.g / 255) as f32
  }

  pub fn get_nb(&self) -> f32 {
    (self.b / 255) as f32
  }

  pub fn get_na(&self) -> f32 {
    (self.a / 255) as f32
  }

  pub fn norm(&self) -> [f32; 4] {
    [self.get_nr(), self.get_ng(), self.get_nb(), self.get_na()]
  }
}

impl Luable for Color {
  fn as_lua(&self, lua: &Lua) -> Result<Value, Box<dyn Error>> {
    let table = lua.create_table()?;

    table.set("r", Value::Integer(self.r as i64))?;
    table.set("g", Value::Integer(self.g as i64))?;
    table.set("b", Value::Integer(self.b as i64))?;
    table.set("a", Value::Integer(self.a as i64))?;

    Ok(Value::Table(table))
  }

  fn from_lua(&mut self, value: Value) -> Result<(), Box<dyn Error>> {
    if let Some(tbl) = value.as_table() {
      self.r = tbl.get("r")?;
      self.g = tbl.get("g")?;
      self.b = tbl.get("b")?;
      self.a = tbl.get("a")?;
      return Ok(());
    }

    Err("Invalid Lua value".into())
  }
}