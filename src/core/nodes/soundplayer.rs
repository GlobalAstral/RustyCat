use std::{collections::HashMap, sync::Mutex};

use futures::{executor::block_on};
use macroquad::audio::{PlaySoundParams, Sound, load_sound, play_sound};
use mlua::{AnyUserData, Function, IntoLua, Table, UserData, Value};
use once_cell::sync::Lazy;

use crate::core::{core::{Downcastable, Luable}, nodelike::NodeLike, nodes::node::Node};

static AUDIO_MANAGER: Lazy<Mutex<HashMap<String, Sound>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub struct SoundPlayer {
  base: Node,
  sound: String
}

impl SoundPlayer {
  pub fn new(sound: &str) -> SoundPlayer {
    let temp = sound.to_string();
    let audio = block_on( load_sound(&temp) ).expect(&format!("Cannot load sound {}", temp));
    AUDIO_MANAGER.lock().as_mut().expect("Failed to get AudioManager").insert(
      temp.clone(),
      audio
    );
    SoundPlayer {
      base: Node::new(),
      sound: sound.to_string()
    }
  }

  pub fn empty() -> SoundPlayer {
    SoundPlayer { base: Node::new(), sound: String::new() }
  }
}

impl NodeLike for SoundPlayer {
  fn get_kind(&self) -> &str {
    "SoundPlayer"
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

impl Luable for SoundPlayer {
  fn as_lua(&self, lua: &mlua::Lua) -> Result<mlua::Value, Box<dyn std::error::Error>> {
    let table = lua.create_table()?;
    table.set("base", self.base.as_lua(lua)?)?;
    table.set("sound", self.sound.clone())?;
    Node::add_kind_to_lua(self.get_kind().to_string(), &table, lua)?;
    table.set("play", lua.create_function(|_, (this, looped, volume): (Table, bool, f32)| {
      let sound: String = this.get::<String>("sound")?;
      let manager = AUDIO_MANAGER.lock().unwrap();
      let tmp = manager.get(&sound).unwrap();
      play_sound(
        tmp,
        PlaySoundParams { looped: looped, volume: volume }
      );
      Ok(())
    })?)?;
    Ok(Value::Table(table)) 
  }

  fn from_lua(&mut self, value: mlua::Value) -> Result<(), Box<dyn std::error::Error>> {
    let table = value.as_table().ok_or("Invalid Lua Value")?;
    self.base.from_lua(table.get("base")?)?;
    self.sound = table.get("sound")?;
    Ok(())
  }
}

impl Downcastable for SoundPlayer {
  fn as_any(&mut self) -> &mut dyn std::any::Any {
    self
  }
}
