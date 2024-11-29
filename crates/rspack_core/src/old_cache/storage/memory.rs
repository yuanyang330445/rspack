use std::collections::HashMap;

use dashmap::DashMap;
use deepsize::DeepSizeOf;
use rspack_collections::{Identifier, IdentifierDashMap};

use super::Storage;

#[derive(Debug)]
pub struct MemoryStorage<Item> {
  pub data: IdentifierDashMap<Item>,
}

impl<Item> MemoryStorage<Item> {
  pub fn new() -> Self {
    Self {
      data: DashMap::default(),
    }
  }
}

impl<Item> Storage<Item> for MemoryStorage<Item>
where
  Item: Clone + std::fmt::Debug + Send + Sync + DeepSizeOf,
{
  fn get(&self, id: &Identifier) -> Option<Item> {
    self.data.get(id).map(|item| item.clone())
  }
  fn set(&self, id: Identifier, data: Item) {
    self.data.insert(id, data);
  }
  fn remove(&self, id: &Identifier) {
    self.data.remove(id);
  }

  fn keys(&self) -> Vec<String> {
    let mut k = self
      .data
      .iter()
      .map(|item| item.key().clone().to_string())
      .collect::<Vec<_>>();

    let a = self
      .data
      .iter()
      .map(|item| (item.key().clone().to_string(), item.value().clone()))
      .collect::<HashMap<_, _>>();

    k.push(a.deep_size_of().to_string());

    k
  }
}
