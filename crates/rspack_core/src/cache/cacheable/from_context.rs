use std::{any::Any, sync::Arc};

use rspack_cacheable::{cacheable, with::AsConverter, DeserializeError, SerializeError};

use super::CacheContext;
use crate::CompilerOptions;

#[cacheable]
pub struct FromContext;

impl AsConverter<Arc<CompilerOptions>> for FromContext {
  fn serialize(_data: &Arc<CompilerOptions>, _ctx: &dyn Any) -> Result<Self, SerializeError> {
    Ok(FromContext)
  }
  fn deserialize(self, ctx: &dyn Any) -> Result<Arc<CompilerOptions>, DeserializeError> {
    let Some(ctx) = ctx.downcast_ref::<CacheContext>() else {
      return Err(DeserializeError::NoContext);
    };
    Ok(ctx.options.clone())
  }
}
