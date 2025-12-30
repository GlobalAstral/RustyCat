
use macroquad::{window::Conf};

use crate::core::{color::Color, core::WindowConfig, engine::Engine, image::Img, nodes::{clickable_area::ClickableArea, rectmesh::RectMesh, sprite::Sprite}, vec2::Vec2};

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

  let test = Sprite::new(Vec2::new(100, 100), Vec2::new(256, 256), Img::new("test.png").with_degrees(45.0).section(Vec2::new(50, 50)));

  engine.children.add_child("test".into(), Box::new(test));

  engine.mainloop().await;
}
