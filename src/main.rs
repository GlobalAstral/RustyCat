
use std::{env, error::Error};

use macroquad::{window::Conf};

use crate::core::{color::Color, core::WindowConfig, engine::Engine, nodes::{clickable_area::ClickableArea, rectmesh::RectMesh, sprite::Sprite}, vec2::Vec2};

mod core;

//? IDK HOW TO CHANGE ICON!!
//TODO SoundPlayer, Button and maybe more Nodes that idk rn

fn get_conf() -> Conf { 
  let args: Vec<String> = env::args().collect();
  let fname: &str = if args.len() <= 1 {
    "main.lua"
  } else {
    args.get(1).unwrap()
  };
  WindowConfig::load(fname).expect(&format!("Cannot load {}", fname)).into()
}

#[macroquad::main(get_conf)]
async fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  let fname: &str = if args.len() <= 1 {
    "main.lua"
  } else {
    args.get(1).unwrap()
  };
  
  let mut engine: Engine = Engine::load(fname)?;


  engine.mainloop().await;
  Ok(())
}
