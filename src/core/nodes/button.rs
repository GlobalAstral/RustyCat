use mlua::Value;

use crate::core::{color::Color, core::{Downcastable, Luable}, image::Img, nodelike::NodeLike, nodes::{clickable_area::ClickableArea, node::Node, sprite::Sprite, text::Text}, vec2::Vec2};


pub struct TextButton {
  base: Node,
  text: Text,
  area: ClickableArea
}

impl TextButton {
  pub fn new(text: &str, pos: Vec2, size: u16, color: Color) -> TextButton {
    let temp = Text::new(text, pos, size, color);
    let size = (&temp).getTextSize();
    TextButton { base: Node::new(), text: temp, area: ClickableArea::new(pos, size) }
  }
}

impl NodeLike for TextButton {
  fn get_kind(&self) -> &str {
    "TextButton"
  }
  fn get_scripts(&mut self) -> &mut crate::core::script_manager::ScriptManager {
    self.base.get_scripts()
  }
  fn load_scripts(&mut self) {
    let tmp = self.get_kind().to_string();
    self.base.load_scripts(&tmp);
    self.area.load_scripts();
    self.text.load_scripts();
  }
  fn render(&mut self) {
    self.base.render();
    self.area.render();
    self.text.render();
  }
  fn setup(&mut self) {
    self.base.setup();
    self.area.setup();
    self.text.setup();
  }
  fn update(&mut self, deltatime: f32) {
    self.base.update(deltatime);
    self.area.update(deltatime);
    self.text.update(deltatime);
    self.area.transform.size = self.text.getTextSize();
  }
}

impl Downcastable for TextButton {
  fn as_any(&mut self) -> &mut dyn std::any::Any {
    self
  }
}

impl Luable for TextButton {
  fn as_lua(&self, lua: &mlua::Lua) -> Result<mlua::Value, Box<dyn std::error::Error>> {
    let table = lua.create_table()?;

    table.set("base", self.base.as_lua(lua)?)?;
    table.set("area", self.area.as_lua(lua)?)?;
    table.set("text", self.text.as_lua(lua)?)?;

    Node::add_kind_to_lua(self.get_kind().to_string(), &table, lua)?;

    Ok(Value::Table(table))
  }
  fn from_lua(&mut self, value: Value) -> Result<(), Box<dyn std::error::Error>> {
    let table = value.as_table().ok_or("Invalid Lua Value")?;
    self.base.from_lua(table.get("base")?)?;
    self.area.from_lua(table.get("area")?)?;
    self.text.from_lua(table.get("text")?)?;
    Ok(())
  }
}


pub struct SpriteButton {
  base: Node,
  sprite: Sprite,
  area: ClickableArea
}

impl SpriteButton {
  pub fn new(pos: Vec2, size: Vec2, img: Img) -> SpriteButton {
    SpriteButton { base: Node::new(), sprite: Sprite::new(pos, size, img), area: ClickableArea::new(pos, size) }
  }
}

impl NodeLike for SpriteButton {
  fn get_kind(&self) -> &str {
    "SpriteButton"
  }
  fn get_scripts(&mut self) -> &mut crate::core::script_manager::ScriptManager {
    self.base.get_scripts()
  }
  fn load_scripts(&mut self) {
    let tmp = self.get_kind().to_string();
    self.base.load_scripts(&tmp);
    self.area.load_scripts();
    self.sprite.load_scripts();
  }
  fn render(&mut self) {
    self.base.render();
    self.area.render();
    self.sprite.render();
  }
  fn setup(&mut self) {
    self.base.setup();
    self.area.setup();
    self.sprite.setup();
  }
  fn update(&mut self, deltatime: f32) {
    self.base.update(deltatime);
    self.area.update(deltatime);
    self.sprite.update(deltatime);
  }
}

impl Downcastable for SpriteButton {
  fn as_any(&mut self) -> &mut dyn std::any::Any {
    self
  }
}

impl Luable for SpriteButton {
  fn as_lua(&self, lua: &mlua::Lua) -> Result<mlua::Value, Box<dyn std::error::Error>> {
    let table = lua.create_table()?;

    table.set("base", self.base.as_lua(lua)?)?;
    table.set("area", self.area.as_lua(lua)?)?;
    table.set("sprite", self.sprite.as_lua(lua)?)?;

    Node::add_kind_to_lua(self.get_kind().to_string(), &table, lua)?;

    Ok(Value::Table(table))
  }
  fn from_lua(&mut self, value: Value) -> Result<(), Box<dyn std::error::Error>> {
    let table = value.as_table().ok_or("Invalid Lua Value")?;
    self.base.from_lua(table.get("base")?)?;
    self.area.from_lua(table.get("area")?)?;
    self.sprite.from_lua(table.get("sprite")?)?;
    Ok(())
  }
}
