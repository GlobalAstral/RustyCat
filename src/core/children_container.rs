use std::{collections::HashMap, hash::Hash};

use crate::core::core::Downcastable;

#[derive(Debug, Clone)]
pub struct ChildrenContainer<K, T> where K: Eq + Hash, T: Downcastable {
  pub children: HashMap<K, T>,
}

impl<K, T> ChildrenContainer<K, T> where K: Eq + Hash, T: Downcastable {
  pub fn new() -> Self {
    Self { children: HashMap::new() }
  }
  pub fn add_child(&mut self, id: K, child: T) {
    self.children.insert(id, child);
  }
  pub fn clear_children(&mut self) {
    self.children.clear();
  }
  pub fn remove_child(&mut self, id: K) {
    let _ = self.children.remove(&id);
  }
  pub fn foreach_child<F>(&mut self, mut func: F) where F: FnMut(&Self, &K, &mut T) {
    let mut tmp: HashMap<K, T> = std::mem::take(&mut self.children);
    tmp.iter_mut().for_each(|(id, node)| {
      func(self, id, node);
    });
    self.children = tmp;
  }
  pub fn get_child<C>(&mut self, id: K) -> Option<&mut C> where C: 'static {
    self.children
        .iter_mut()
        .find(|(key, _)| *key == &id)
        .and_then(|(_, child)| child.as_any().downcast_mut::<C>())
  }
}
