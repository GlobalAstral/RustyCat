use std::{any::Any, error::Error, path::PathBuf};

use image::GenericImageView;
use macroquad::{texture::Image, window::Conf};
use mlua::{Lua, Value};
use crate::core::vec2::Vec2;

#[derive(Debug)]
pub struct WindowConfig {
  pub title: String,
  pub size: Vec2,
  pub fullscreen: bool,
  pub resizable: bool,
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

pub fn loadImage(path: PathBuf) -> Image {
  let img = image::open(path).expect("Cannot load image");
  let (w, h) = img.dimensions();
  let rgba = img.to_rgba8().to_vec();
  Image {
    width: w as u16,
    height: h as u16,
    bytes: rgba
  }
}

pub trait Luable {
  fn as_lua(&mut self, lua: &Lua) -> Result<Value, Box<dyn Error>>;
  fn from_lua(&mut self, value: Value) -> Result<(), Box<dyn Error>>;
}

pub trait Downcastable {
  fn as_any(&mut self) -> &mut dyn Any;
}
