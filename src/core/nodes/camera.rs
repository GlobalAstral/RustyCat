use mlua::Value;

use crate::core::{core::{Downcastable, Luable}, keys, nodelike::NodeLike, nodes::node::Node, transform::Transform, vec2::Vec2};


pub struct Camera {
  base: Node,
  pub focal_length: f32,
  pub transform: Transform
}

impl Camera {
  pub fn new(pos: Vec2, surface: Vec2, focal_length: f32) -> Camera {
    Camera { base: Node::new(), focal_length, transform: Transform::new(pos, surface) }
  }
}

impl NodeLike for Camera {
  fn get_kind(&self) -> &str {
    "Camera"
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

impl Luable for Camera {
  fn as_lua(&mut self, lua: &mlua::Lua) -> Result<mlua::Value, Box<dyn std::error::Error>> {
    let table = lua.create_table()?;

    table.set("base", self.base.as_lua(lua)?)?;
    table.set("focal_length", self.focal_length)?;
    table.set("transform", self.transform.as_lua(lua)?)?;

    Node::add_kind_to_lua(self.get_kind().to_string(), &table, lua)?;

    Ok(Value::Table(table))
  }
  fn from_lua(&mut self, value: mlua::Value) -> Result<(), Box<dyn std::error::Error>> {
    let table = value.as_table().ok_or("Invalid Lua Value".to_string())?;
    self.base.from_lua(table.get("base")?)?;
    self.transform.from_lua(table.get("transform")?)?;
    self.focal_length = table.get("focal_length")?;
    Ok(())
  }
}

impl Downcastable for Camera {
  fn as_any(&mut self) -> &mut dyn std::any::Any {
    self
  }
}
