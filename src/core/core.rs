use std::{any::Any, error::Error, f32::consts::PI, fs, path::PathBuf};

use image::GenericImageView;
use macroquad::{input::{KeyCode, MouseButton, is_key_down, is_key_pressed, is_key_released, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released}, miniquad::window, texture::{DrawTextureParams, Image, Texture2D, load_texture}, window::Conf};
use mlua::{AnyUserData, Chunk, Lua, MultiValue, Table, Value};
use crate::core::{color::Color, image::Img, keys::Stringable, nodelike::NodeLike, nodes::{clickable_area::ClickableArea, node::Node, rectmesh::RectMesh, sprite::Sprite, text::Text}, script_manager::{ScriptManager, ScriptManagerSecret}, transform::Transform, vec2::Vec2};

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

pub fn call_constructor(kind: &str, node: Value) -> Result<Box<dyn NodeLike>, mlua::Error> {
  Ok(match kind {
    "Node" => {
      let mut tmp: Node = Node::new();
      tmp.from_lua(node).expect("Invalid Lua Value");
      Box::new(tmp)
    },
    "RectMesh" => {
      let mut tmp: RectMesh = RectMesh::new(Vec2::ZERO, Vec2::ZERO, Color::new(0));
      tmp.from_lua(node).expect("Invalid Lua Value");
      Box::new(tmp)
    },
    "ClickableArea" => {
      let mut tmp: ClickableArea = ClickableArea::new(Vec2::ZERO, Vec2::ZERO);
      tmp.from_lua(node).expect("Invalid Lua Value");
      Box::new(tmp)
    },
    "Sprite" => {
      let mut tmp: Sprite = Sprite::new(Vec2::ZERO, Vec2::ZERO, Img::new(""));
      tmp.from_lua(node).expect("Invalid Lua Value");
      Box::new(tmp)
    },
    "Text" => {
      let mut tmp: Text = Text::new("", Vec2::ZERO, 0, Color::new(0));
      tmp.from_lua(node).expect("Invalid Lua Value");
      Box::new(tmp)
    },
    _ => {
      return Err(mlua::Error::RuntimeError("Node not recognized".into()))
    }
  })
}

pub fn init_env_commons(lua: &Lua, env: &Table) -> Result<(), Box<dyn Error>> {
  env.set("print", lua.create_function(|_, mut args: MultiValue| {
    let mut default_sep = ", ".to_string();
    let mut default_end = "\n".to_string();
    if let Some(Value::String(s)) = args.iter().last() {
      let s = s.to_str()?;
      if s.starts_with("sep=") {
        default_sep = s[4..].to_string();
        args.pop_back();
      }
    }

    if let Some(Value::String(s)) = args.iter().last() {
      let s = s.to_str()?;
      if s.starts_with("end=") {
        default_end = s[4..].to_string();
        args.pop_back();
      }
    }
    let parts: Vec<String> = args.iter().map(|ele| {ScriptManager::stringify(ele, 0)}).collect();
    print!("{}{}", parts.join(&default_sep), default_end);
    Ok(())
  })?)?;

  env.set("keydown", lua.create_function(|_, key: String| {
    let code: Option<Box<KeyCode>> = KeyCode::from_string(&key);
    if let Some(keycode) = code {
      return Ok(is_key_down(*keycode))
    }
    Ok(false)
  })?)?;

  env.set("keypressed", lua.create_function(|_, key: String| {
    let code: Option<Box<KeyCode>> = KeyCode::from_string(&key);
    if let Some(keycode) = code {
      return Ok(is_key_pressed(*keycode))
    }
    Ok(false)
  })?)?;

  env.set("keyreleased", lua.create_function(|_, key: String| {
    let code: Option<Box<KeyCode>> = KeyCode::from_string(&key);
    if let Some(keycode) = code {
      return Ok(is_key_released(*keycode))
    }
    Ok(false)
  })?)?;

  env.set("mkeydown", lua.create_function(|_, key: i64| {
    let button = match key {
      0 => MouseButton::Left,
      1 => MouseButton::Right,
      2 => MouseButton::Middle,
      _ => {
        return Ok(false);
      }
    };
    Ok(is_mouse_button_down(button))
  })?)?;

  env.set("mkeypressed", lua.create_function(|_, key: i64| {
    let button = match key {
      0 => MouseButton::Left,
      1 => MouseButton::Right,
      2 => MouseButton::Middle,
      _ => {
        return Ok(false);
      }
    };
    Ok(is_mouse_button_pressed(button))
  })?)?;

  env.set("mkeyreleased", lua.create_function(|_, key: i64| {
    let button = match key {
      0 => MouseButton::Left,
      1 => MouseButton::Right,
      2 => MouseButton::Middle,
      _ => {
        return Ok(false);
      }
    };
    Ok(is_mouse_button_released(button))
  })?)?;

  env.set("Vec2", lua.create_function(|this, (x, y) : (i32, i32)| {
    Ok(Vec2::new(x, y).as_lua(this).expect("Cannot convert to Vec2"))
  })?)?;

  env.set("Transform", lua.create_function(|this, (pos, size) : (Table, Table)| {
    let mut position = Vec2::ZERO.clone();
    let mut sz = Vec2::ZERO.clone();
    position.from_lua(Value::Table(pos)).expect("Cannot convert to Vec2");
    sz.from_lua(Value::Table(size)).expect("Cannot convert to Vec2");
    Ok(Transform::new(position, sz).as_lua(this).expect("Cannot convert to Transform"))
  })?)?;

  env.set("Img", lua.create_function(|this, (tex, rot, src, tint, fx, fy) : (String, f32, Value, Value, bool, bool)| {
    let mut im = Img::new(&tex)
    .with_degrees(rot)
    .flip(fx, fy);
    
    if src.is_table() {
      let mut v = Vec2::ZERO.clone();
      v.from_lua(src).expect("Cannot convert to Vec2");
      im = im.section(v);
    }

    if tint.is_table() {
      let mut col = Color::new(0);
      col.from_lua(tint).expect("Cannot convert to Color");
      im = im.tint(col);
    }

    Ok(im.as_lua(this).expect("Cannot convert to Color"))
  })?)?;

  env.set("ColorRgba", lua.create_function(|this, (r, g, b, a) : (u8, u8, u8, u8)| {
    let mut col = Color::from_rgba(r, g, b, a);
    Ok(col.as_lua(this).expect("Cannot convert to Color"))
  })?)?;

  env.set("ColorRgb", lua.create_function(|this, (r, g, b) : (u8, u8, u8)| {
    let mut col = Color::from_rgb(r, g, b);
    Ok(col.as_lua(this).expect("Cannot convert to Color"))
  })?)?;

  env.set("Color", lua.create_function(|this, i: u32| {
    let mut col = Color::new(i);
    Ok(col.as_lua(this).expect("Cannot convert to Color"))
  })?)?;

  env.set("ColorHex", lua.create_function(|this, s: String| {
    let mut col = Color::from_hex(&s);
    Ok(col.as_lua(this).expect("Cannot convert to Color"))
  })?)?;

  env.set("Node", lua.create_function(|this, ()| {
    Ok(Node::new().as_lua(this).expect("Cannot convert Node to Lua Value"))
  })?)?;

  env.set("RectMesh", lua.create_function(|this, (pos, sz, col) : (Table, Table, Table)| {
    let mut position: Vec2 = Vec2::ZERO.clone();
    let mut size: Vec2 = Vec2::ZERO.clone();
    let mut color: Color = Color::new(0);
    position.from_lua(Value::Table(pos)).expect("Invalid Lua Value");
    size.from_lua(Value::Table(sz)).expect("Invalid Lua Value");
    color.from_lua(Value::Table(col)).expect("Invalid Lua Value");
    Ok(RectMesh::new(position, size, color).as_lua(this).expect("Cannot convert RectMesh to Lua Value"))
  })?)?;

  env.set("ClickableArea", lua.create_function(|this, (pos, sz) : (Table, Table)| {
    let mut position: Vec2 = Vec2::ZERO.clone();
    let mut size: Vec2 = Vec2::ZERO.clone();
    position.from_lua(Value::Table(pos)).expect("Invalid Lua Value");
    size.from_lua(Value::Table(sz)).expect("Invalid Lua Value");
    Ok(ClickableArea::new(position, size).as_lua(this).expect("Cannot convert ClickableArea to Lua Value"))
  })?)?;

  env.set("Sprite", lua.create_function(|this, (pos, sz, img) : (Table, Table, Table)| {
    let mut position: Vec2 = Vec2::ZERO.clone();
    let mut size: Vec2 = Vec2::ZERO.clone();
    let mut im: Img = Img::new("");
    position.from_lua(Value::Table(pos)).expect("Invalid Lua Value");
    size.from_lua(Value::Table(sz)).expect("Invalid Lua Value");
    im.from_lua(Value::Table(img)).expect("Invalid Lua Value");
    Ok(Sprite::new(position, size, im).as_lua(this).expect("Cannot convert Sprite to Lua Value"))
  })?)?;

  env.set("Text", lua.create_function(|this, (text, pos, size, col): (String, Table, u16, Table)| {
    let mut position: Vec2 = Vec2::ZERO.clone();
    position.from_lua(Value::Table(pos)).expect("Invalid Lua Value");

    let mut color: Color = Color::new(0);
    color.from_lua(Value::Table(col)).expect("Invalid Lua Value");

    Ok(Text::new(&text, position, size, color).as_lua(this).expect("Cannot convert Text to Lua Value"))
  })?)?;

  env.set("embed", lua.create_function_mut(|this, (script, node): (String, Table)| {
    let scripts: AnyUserData = match node.get::<Table>("base") {
      Ok(tbl) => {
        tbl.get::<AnyUserData>("scripts")?
      },
      Err(e) => match node.get::<AnyUserData>("scripts") {
        Ok(tbl) => tbl,
        Err(err) => {
          return Err(mlua::Error::RuntimeError(format!("Seriously, dude. How did you even crash this?\n {}\n\n{}", e, err).into()))
        }
      }
    };

    let scripts: Table = ScriptManagerSecret::from_userdata(scripts).expect("Cannot get scripts");
    
    let later: Table = node.clone();
    let environment: Table = ScriptManager::create_environment(this, Value::Table(node)).expect("Cannot create environment");
        
    this.load(fs::read_to_string(&script)?)
    .set_environment(environment.clone())
    .set_name(script)
    .exec()?;
    scripts.push(environment)?;
    
    Ok(later)
  })?)?;

  env.set("quit", lua.create_function(|_, ()| {
    window::request_quit();
    Ok(())
  })?)?;

  Ok(())
}
