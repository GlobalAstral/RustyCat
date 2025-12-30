use std::{any::Any, error::Error};

use mlua::{Lua, Table, Value};

use crate::core::{children_container::ChildrenContainer, core::{Downcastable, Luable}, nodelike::{NodeLike, generate_id}, script_manager::ScriptManager};

pub struct Node {
  id: u64,
  children: ChildrenContainer<String, Box<dyn NodeLike>>,
  scripts: ScriptManager
}

impl Node {
  pub fn new() -> Node {
    Node { id: generate_id(), children: ChildrenContainer::new(), scripts: ScriptManager::new() }
  }
  fn render_children(&mut self) {
    self.children.foreach_child(|_, _, nodelike| {
      nodelike.render();
    });
  }
  
  fn update_children(&mut self, dt: f32) {
    self.children.foreach_child(|_, _, nodelike| {
      nodelike.update(dt);
    });
  }
  fn setup_children(&mut self) {
    self.children.foreach_child(|_, _, nodelike| {
      nodelike.setup();
    });
  }
  pub fn load_scripts(&mut self, node_name: &str) {
    self.scripts.loadScripts().expect(&format!("Cannot load scripts for {}<{}>", node_name, self.id));
  }
}

impl NodeLike for Node {
  fn setup(&mut self) {
    self.setup_children();
  }
  fn render(&mut self) {
    self.render_children();
  }
  fn update(&mut self, deltatime: f32) {
    self.update_children(deltatime);
  }
  fn load_scripts(&mut self) {
    self.load_scripts("Node");
  }
  fn get_scripts(&mut self) -> &mut ScriptManager {
    &mut self.scripts
  }
}

impl Luable for Node {
  fn as_lua(&mut self, lua: &Lua) -> Result<Value, Box<dyn Error>> {
    let table = lua.create_table()?;
    let children: Table = lua.create_table()?;
    let moved = std::mem::take(&mut self.children.children);
    for (id, mut node) in moved {
      let tmp = node.as_lua(lua).unwrap();
      children.set(id.clone(), tmp)?;
    }
    table.set("children", children)?;
    let id = self.id;
    table.set("id", lua.create_function(move |_, ()| {
      Ok(id)
    })?)?;

    Ok(Value::Table(table))
  }

  fn from_lua(&mut self, value: Value) -> Result<(), Box<dyn Error>> {
    if let Some(tbl) = value.as_table() {
      let children: Table = tbl.get("children")?;
      if children.len()? > self.children.children.len() as i64 {
        return Err("Cannot add Children in raw Lua".into());
      }
      let mut pairs: Vec<(String, Value)> = Vec::new();
      children.for_each(|k, v| {
        pairs.push((k, v));
        Ok(())
      })?;

      for i in 0..children.len()? {
        let pair = pairs.iter().nth(i as usize).unwrap();
        self.children.children.get_mut(&pair.0).unwrap().from_lua(pair.1.clone())?;
      }

      return Ok(())
    }

    Err("Invalid Lua Value".into())
  }
}

impl Downcastable for Node {
  fn as_any(&mut self) -> &mut dyn Any {
    self
  }
}
