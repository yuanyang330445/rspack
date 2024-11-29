use std::fmt::Debug;

use deepsize::DeepSizeOf;
use rspack_collections::Identifier;

use crate::CacheOptions;

mod memory;
pub use memory::MemoryStorage;

pub trait Storage<Item: DeepSizeOf>: Debug + Send + Sync {
  fn get(&self, id: &Identifier) -> Option<Item>;
  fn set(&self, id: Identifier, data: Item);
  fn remove(&self, id: &Identifier);
  fn keys(&self) -> Vec<String>;
  // fn begin_idle(&self);
  // fn end_idle(&self);
  // fn clear(&self);
}

pub fn new_storage<Item>(options: &CacheOptions) -> Option<Box<dyn Storage<Item>>>
where
  Item: Debug + Clone + Send + Sync + 'static + DeepSizeOf,
{
  match options {
    CacheOptions::Disabled => None,
    _ => Some(Box::new(MemoryStorage::new())),
  }
}
