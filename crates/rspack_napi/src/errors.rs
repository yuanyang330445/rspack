use napi::{bindgen_prelude::*, Env, Error, JsUnknown, NapiValue, Result};
use rspack_error::NodeError;

pub trait NapiErrorExt {
  fn into_rspack_error(self) -> rspack_error::Error;
  fn into_rspack_error_with_detail(self, env: &Env) -> rspack_error::Error;
}

pub trait NapiResultExt<T> {
  fn into_rspack_result(self) -> rspack_error::Result<T>;
  fn into_rspack_result_with_detail(self, env: &Env) -> rspack_error::Result<T>;
}

impl NapiErrorExt for Error {
  fn into_rspack_error(self) -> rspack_error::Error {
    (NodeError {
      reason: self.reason,
      stack: None,
      hide_stack: None,
    })
    .into()
  }
  fn into_rspack_error_with_detail(self, env: &Env) -> rspack_error::Error {
    let extract_stack_or_message_from_napi_error =
      extract_stack_or_message_from_napi_error(env, self);

    match extract_stack_or_message_from_napi_error {
      Ok((reason, stack, hide_stack)) => (NodeError {
        reason,
        stack,
        hide_stack,
      })
      .into(),
      Err(e) => (NodeError {
        reason: format!("Unknown Error when extracting from napi error: {e}"),
        stack: None,
        hide_stack: None,
      })
      .into(),
    }
  }
}

impl<T: 'static> NapiResultExt<T> for Result<T> {
  fn into_rspack_result(self) -> rspack_error::Result<T> {
    self.map_err(|e| e.into_rspack_error())
  }
  fn into_rspack_result_with_detail(self, env: &Env) -> rspack_error::Result<T> {
    self.map_err(|e| e.into_rspack_error_with_detail(env))
  }
}

/// Extract stack or message from a native Node error object,
/// otherwise we try to format the error from the given `Error` object that indicates which was created on the Rust side.
#[inline(always)]
fn extract_stack_or_message_from_napi_error(
  env: &Env,
  err: Error,
) -> Result<(String, Option<String>, Option<bool>)> {
  let maybe_reason = err.reason.clone();
  let napi_value = unsafe { ToNapiValue::to_napi_value(env.raw(), err) }?;
  // try object
  match unsafe { JsUnknown::from_raw_unchecked(env.raw(), napi_value) }.coerce_to_object() {
    Ok(napi_error) => {
      let hide_stack = napi_error.get_named_property::<Option<bool>>("hideStack")?;
      let stack = napi_error.get_named_property::<Option<String>>("stack")?;

      // This is intended to be different than webpack,
      // here we want to treat the almost the same as `Error.stack` just without the stack.
      // Webpack uses `Error.message`, however it does not contain the `Error.prototype.name`
      // `xxx` -> `Error: xxx`. So they behave the same even if `hideStack` is set to `true`.
      let to_string = napi_error.get_named_property::<JsFunction>("toString")?;
      let message = to_string.apply0::<String, JsUnknown>(napi_error.into_unknown())?;

      Ok((
        if hide_stack.unwrap_or_default() {
          message
        } else {
          stack.clone().unwrap_or(message)
        },
        stack,
        hide_stack,
      ))
    }
    Err(_) => {
      // try string
      match unsafe { JsUnknown::from_raw_unchecked(env.raw(), napi_value) }.coerce_to_string() {
        Ok(s) => Ok((s.into_utf8()?.as_str()?.to_owned(), None, None)),
        Err(_) => Ok((maybe_reason, None, None)),
      }
    }
  }
}
