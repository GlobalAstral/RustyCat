use macroquad::input::mouse_position;
use mlua::Value;

use crate::core::{core::{Downcastable, Luable}, nodelike::NodeLike, nodes::node::Node, script_manager::ScriptManager, transform::Transform, vec2::Vec2};


pub struct ClickableArea {
  base: Node,
  transform: Transform
}

impl ClickableArea {
  pub fn new(pos: Vec2, size: Vec2) -> Self {
    ClickableArea { base: Node::new(), transform: Transform::new(pos, size) }
  }
}

impl NodeLike for ClickableArea {
  fn setup(&mut self) {
    self.base.setup();
  }
  fn update(&mut self, deltatime: f32) {
    self.base.update(deltatime);
  }
  fn get_scripts(&mut self) -> &mut ScriptManager {
    self.base.get_scripts()
  }
  fn load_scripts(&mut self) {
    let tmp = self.get_kind().to_string();
    self.base.load_scripts(&tmp);
  }
  fn render(&mut self) {
    self.base.render();
  }
  fn get_kind(&self) -> &str {
    "ClickableArea"
  }
}

impl Luable for ClickableArea {
  fn as_lua(&mut self, lua: &mlua::Lua) -> Result<mlua::Value, Box<dyn std::error::Error>> {
    let tbl = lua.create_table()?;
    let base = self.base.as_lua(lua)?;
    let transform = self.transform.as_lua(lua)?;

    tbl.set("base", base)?;
    tbl.set("transform", transform)?;
    let temp = self.transform.clone();
    tbl.set("clicked", lua.create_function(move |_, s: i64| {
      let (x, y) = mouse_position();
      let mouse = Vec2::new(x as i32, y as i32);
      let inside = temp.contains(mouse);
      let pressed = match s {
          0 => macroquad::input::is_mouse_button_pressed(macroquad::input::MouseButton::Left),
          1 => macroquad::input::is_mouse_button_pressed(macroquad::input::MouseButton::Right),
          2 => macroquad::input::is_mouse_button_pressed(macroquad::input::MouseButton::Middle),
          _ => false,
      };
      Ok(inside && pressed)
    })?)?;
    Node::add_kind_to_lua(self.get_kind().to_string(), &tbl, lua)?;
    Ok(Value::Table(tbl))
  }

  fn from_lua(&mut self, value: Value) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(table) = value.as_table() {
      let base: Value = table.get("base")?;
      self.base.from_lua(base)?;
      let transform: Value = table.get("transform")?;
      self.transform.from_lua(transform)?;
      return Ok(())
    }

    Err("Invalid Lua Value".into())
  }
}

impl Downcastable for ClickableArea {
  fn as_any(&mut self) -> &mut dyn std::any::Any {
    self
  }
}
