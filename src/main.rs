
use macroquad::{window::Conf};

use crate::core::{color::Color, core::WindowConfig, engine::Engine, nodes::rectmesh::RectMesh, vec2::Vec2};

mod core;

//? IDK HOW TO CHANGE ICON!!

fn get_conf() -> Conf { 
  WindowConfig {
    title: "Masters of Souls".into(),
    fullscreen: false,
    size: Vec2::new(1280, 720),
    resizable: true,
  }.into()
}

#[macroquad::main(get_conf)]
async fn main() {
  let mut engine: Engine = Engine::new(Color::new(0xFF000000));

  let mut test: RectMesh = RectMesh::new(Vec2::new(100, 100), Vec2::new(100, 100), Color::new(0xFFFF0000));

  engine.add_script_to_node(&mut test, "temp.lua");

  engine.children.add_child("test".into(), Box::new(test));

  engine.mainloop().await;
}
