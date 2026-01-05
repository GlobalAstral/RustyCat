use futures::executor::block_on;
use macroquad::text::{Font, TextDimensions, TextParams, draw_text_ex, load_ttf_font, measure_text};
use mlua::{AnyUserData, IntoLua, Table, UserData, Value};

use crate::core::{color::Color, core::{Downcastable, Luable}, engine::main_camera, nodelike::NodeLike, nodes::node::Node, transform::Transform, vec2::Vec2};

pub struct Text {
  base: Node,
  text: String,
  pos: Vec2,
  scale: f32,
  aspect: f32,
  font_size: u16,
  font: Option<Font>,
  font_path: Option<String>,
  rotation: f32,
  color: Color,
}

impl Text {
  pub fn new(text: &str, pos: Vec2, size: u16, color: Color) -> Text {
    Text { 
      base: Node::new(), 
      text: text.to_string(), 
      pos: pos, 
      scale: 1.0, 
      aspect: 1.0, 
      font_size: size, 
      font: None, 
      font_path: None,
      rotation: 0.0, 
      color: color, 
    }
  }

  fn load_font(&mut self) {
    self.font = if self.font_path.is_some() {
      let tmp: &str = &self.font_path.as_ref().unwrap();
      Some( block_on( load_ttf_font(tmp) ).expect(&format!("Cannot load Font {}", tmp)) )
    } else {
      None
    }
  }

  pub fn getTextSize(&self) -> Vec2 {
    let temp = measure_text(
      &self.text, 
      self.font.as_ref(), 
      self.font_size, 
      self.scale, 
    );
    Vec2::new(temp.width as i32, temp.height as i32)
  }
}

impl NodeLike for Text {
  fn get_kind(&self) -> &str {
    "Text"
  }
  fn get_scripts(&mut self) -> &mut crate::core::script_manager::ScriptManager {
    self.base.get_scripts()  
  }
  fn load_scripts(&mut self) {
    let tmp = self.get_kind().to_string();
    self.base.load_scripts(&tmp);
  }
  fn setup(&mut self) {
    self.base.setup();
  }
  fn render(&mut self) {
    self.base.render();

    let scale = if let Some(cam) = main_camera().as_ref() {
      self.scale / cam.focal_length
    } else {
      self.scale
    };

    draw_text_ex(
      &self.text, 
      self.pos.get_x() as f32, 
      self.pos.get_y() as f32, 
      TextParams { 
        font: if self.font.is_some() {
          let tmp = self.font.as_ref().unwrap();
          Some(tmp)
        } else { None }, 
        font_size: self.font_size, 
        font_scale: scale, 
        font_scale_aspect: self.aspect, 
        rotation: self.rotation, 
        color: self.color.into()
      }
    );
  }
  fn update(&mut self, deltatime: f32) {
    self.base.update(deltatime);
  }
}

impl Luable for Text {
  fn as_lua(&self, lua: &mlua::Lua) -> Result<mlua::Value, Box<dyn std::error::Error>> {
    let table = lua.create_table()?;

    table.set("base", self.base.as_lua(lua)?)?;
    table.set("text", self.text.clone())?;
    table.set("pos", self.pos.as_lua(lua)?)?;
    table.set("scale", self.scale)?;
    table.set("aspect", self.aspect)?;
    table.set("font_size", self.font_size)?;
    table.set("rotation", self.rotation)?;
    table.set("color", self.color.as_lua(lua)?)?;
    table.set("font", self.font_path.clone().unwrap_or("".to_string()).into_lua(lua)?)?;

    table.set("dimensions", lua.create_function(|thislua, this: Table| {
      let font = {
        let font_path: String = this.get("font")?;
        if font_path.is_empty() {
          None
        } else {
          Some(&block_on( load_ttf_font(&font_path) ).expect(&format!("Cannot load Font {}", &font_path)))
        }
      };
      let dims = measure_text(
        &this.get::<String>("text")?, 
        font, 
        this.get("font_size")?, 
        this.get("scale")?, 
      );
      Ok(Vec2::new(dims.width as i32, dims.height as i32).as_lua(thislua).expect("Invalid Lua Value"))
    })?)?;
    
    Node::add_kind_to_lua(self.get_kind().to_string(), &table, lua)?;

    Ok(Value::Table(table))
  }
  fn from_lua(&mut self, value: mlua::Value) -> Result<(), Box<dyn std::error::Error>> {
    let table: &Table = value.as_table().ok_or("Invalid Lua Value".to_string())?;

    self.base.from_lua(table.get("base")?)?;
    self.text = table.get::<String>("text")?;
    self.pos.from_lua(table.get("pos")?)?;
    self.scale = table.get("scale")?;
    self.aspect = table.get("aspect")?;
    self.font_size = table.get("font_size")?;
    let old_path = self.font_path.clone().unwrap_or(String::new());
    self.font_path = {
      let tmp = table.get::<String>("font")?;
      if tmp.is_empty() {
        None
      } else {
        Some(tmp)
      }
    };
    if !(self.font_path.clone().is_none() && old_path.is_empty() || self.font_path.clone().unwrap() == old_path) {
      self.load_font();
    }
    self.rotation = table.get("rotation")?;
    self.color.from_lua(table.get("color")?)?;

    Ok(())
  }
}

impl Downcastable for Text {
  fn as_any(&mut self) -> &mut dyn std::any::Any {
    self
  }
}
