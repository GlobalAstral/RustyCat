use std::{any::Any, sync::atomic::{AtomicU64, Ordering}};

use crate::core::{core::{Downcastable, Luable}, script_manager::ScriptManager};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

pub fn generate_id() -> u64 {
  NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

pub trait NodeLike: Downcastable + Luable + Send + Sync {
  fn setup(&mut self);
  fn update(&mut self, deltatime: f32);
  fn render(&mut self);
  fn load_scripts(&mut self);
  fn get_scripts(&mut self) -> &mut ScriptManager;
  fn get_kind(&self) -> &str;
}

impl Downcastable for Box<dyn NodeLike + Send + Sync> {
  fn as_any(&mut self) -> &mut dyn Any {
    self.as_mut().as_any()
  }
}

impl dyn NodeLike {
  pub fn cast<T: 'static>(&mut self) -> Option<&mut T> {
    self.as_any().downcast_mut::<T>()
  }
}
