mod cache;

pub use cache::CacheOptions as ExperimentCacheOptions;

use crate::incremental::IncrementalPasses;

#[derive(Debug)]
pub struct Experiments {
  pub layers: bool,
  pub incremental: IncrementalPasses,
  pub top_level_await: bool,
  pub rspack_future: RspackFuture,
  pub cache: ExperimentCacheOptions,
  pub output_module: bool,
  pub future_defaults: bool,
  pub css: bool,
  pub async_web_assembly: bool,
}

#[allow(clippy::empty_structs_with_brackets)]
#[derive(Debug)]
pub struct RspackFuture {}
