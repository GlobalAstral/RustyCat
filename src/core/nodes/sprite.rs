use mlua::Value;

use crate::core::{core::{Downcastable, Luable}, engine::main_camera, image::Img, nodelike::NodeLike, nodes::node::Node, transform::Transform, vec2::Vec2};


pub struct Sprite {
  base: Node,
  transform: Transform,
  img: Img
}

impl Sprite {
  pub fn new(pos: Vec2, size: Vec2, img: Img) -> Sprite {
    Sprite { base: Node::new(), transform: Transform::new(pos, size), img }
  }
}

impl NodeLike for Sprite {
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
  fn update(&mut self, deltatime: f32) {
    self.base.update(deltatime);
  }
  fn render(&mut self) {
    self.base.render();
    let (actual_position, actual_size): (Vec2, Vec2) = self.transform.get_camera_relative();
    self.img.render(actual_position, actual_size);
  }
  fn get_kind(&self) -> &str {
    "Sprite"
  }
}

impl Luable for Sprite {
  fn as_lua(&mut self, lua: &mlua::Lua) -> Result<mlua::Value, Box<dyn std::error::Error>> {
    let table = lua.create_table()?;
    table.set("base", self.base.as_lua(lua)?)?;
    table.set("transform", self.transform.as_lua(lua)?)?;
    table.set("img", self.img.as_lua(lua)?)?;
    Node::add_kind_to_lua(self.get_kind().to_string(), &table, lua)?;
    Ok(Value::Table(table))
  }

  fn from_lua(&mut self, value: Value) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(table) = value.as_table() {
      self.base.from_lua(table.get("base")?)?;
      self.transform.from_lua(table.get("transform")?)?;
      self.img.from_lua(table.get("img")?)?;
      return Ok(())
    }
    Err("Invalid Lua Value".into())
  }
}

impl Downcastable for Sprite {
  fn as_any(&mut self) -> &mut dyn std::any::Any {
    self
  }
}
