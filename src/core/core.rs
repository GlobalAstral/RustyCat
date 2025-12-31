use std::{any::Any, error::Error, f32::consts::PI, fs, path::PathBuf};

use image::GenericImageView;
use macroquad::{texture::{DrawTextureParams, Image, Texture2D, load_texture}, window::Conf};
use mlua::{Chunk, Lua, Table, Value};
use crate::core::vec2::Vec2;

#[derive(Debug)]
pub struct WindowConfig {
  pub title: String,
  pub size: Vec2,
  pub fullscreen: bool,
  pub resizable: bool,
}

impl WindowConfig {
  pub fn load(path: &str) -> Result<WindowConfig, Box<dyn Error>> {
    let file_content: String = fs::read_to_string(path)?;
    let lua: Lua = Lua::new();
    let chunk: Chunk = lua.load(file_content);
    chunk.exec()?;
    Ok(
      WindowConfig { 
        title: lua.globals().get("Title").unwrap_or("Default Window".to_string()), 
        size: {
          let tmp: Result<Value, mlua::Error> = lua.globals().get("Size");
          if tmp.is_err() {
            Vec2::new(500, 500)
          } else {
            let mut vec: Vec2 = Vec2::ZERO.clone();
            let r: Result<(), Box<dyn Error>> = vec.from_lua(tmp.unwrap());
            if r.is_err() {
              Vec2::new(500, 500)
            } else {
              vec
            }
          }
        }, 
        fullscreen: lua.globals().get("Fullscreen").unwrap_or(false), 
        resizable: lua.globals().get("Resizable").unwrap_or(true), 
      }
    )
  }
}

impl Into<Conf> for WindowConfig {
  fn into(self) -> Conf {
    Conf { 
      window_title: self.title, 
      window_width: self.size.get_x(), 
      window_height: self.size.get_y(),  
      fullscreen: self.fullscreen,  
      window_resizable: self.resizable, 
      ..Default::default()
    }
  }
}

pub fn radians(degrees: f32) -> f32 {
  degrees * PI / 180.0
}

pub trait Luable {
  fn as_lua(&mut self, lua: &Lua) -> Result<Value, Box<dyn Error>>;
  fn from_lua(&mut self, value: Value) -> Result<(), Box<dyn Error>>;
}

pub trait Downcastable {
  fn as_any(&mut self) -> &mut dyn Any;
}
