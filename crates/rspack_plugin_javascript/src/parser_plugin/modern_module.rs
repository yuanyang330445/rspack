use rspack_core::SpanExt;
use swc_core::ecma::ast::{CallExpr, Callee, Expr, Ident, UnaryExpr};

use super::api_plugin::{
  NON_WEBPACK_REQUIRE, RSPACK_UNIQUE_ID, RSPACK_VERSION, SYSTEM_CONTEXT, WEBPACK_BASE_URI,
  WEBPACK_CHUNK_LOAD, WEBPACK_CHUNK_NAME, WEBPACK_GET_SCRIPT_FILENAME, WEBPACK_HASH,
  WEBPACK_INIT_SHARING, WEBPACK_LAYER, WEBPACK_MODULE, WEBPACK_MODULES, WEBPACK_NONCE,
  WEBPACK_PUBLIC_PATH, WEBPACK_REQUIRE, WEBPACK_RUNTIME_ID, WEBPACK_SHARE_SCOPES,
};
use crate::parser_plugin::JavascriptParserPlugin;
use crate::utils::eval::{self, BasicEvaluatedExpression};
use crate::visitors::extract_member_root;
use crate::visitors::{expr_matcher, JavascriptParser};

pub struct ModernModulePlugin;

/// This plugin is implemented based on /crates/rspack_plugin_javascript/src/parser_plugin/api_plugin.rs
/// When building a library of type modern-module, this parser plugin prevents APIPlugin parser plugin
/// from taking effect in advance.
impl ModernModulePlugin {
  pub fn new() -> Self {
    Self
  }
}

fn get_typeof_evaluate_of_api(sym: &str) -> Option<&str> {
  match sym {
    WEBPACK_REQUIRE => Some("function"),
    WEBPACK_HASH => Some("string"),
    WEBPACK_PUBLIC_PATH => Some("string"),
    WEBPACK_MODULES => Some("object"),
    WEBPACK_MODULE => Some("object"),
    WEBPACK_CHUNK_LOAD => Some("function"),
    WEBPACK_BASE_URI => Some("string"),
    NON_WEBPACK_REQUIRE => None,
    SYSTEM_CONTEXT => Some("object"),
    WEBPACK_SHARE_SCOPES => Some("object"),
    WEBPACK_INIT_SHARING => Some("function"),
    WEBPACK_NONCE => Some("string"),
    WEBPACK_CHUNK_NAME => Some("string"),
    WEBPACK_RUNTIME_ID => None,
    WEBPACK_GET_SCRIPT_FILENAME => Some("function"),
    RSPACK_VERSION => Some("string"),
    RSPACK_UNIQUE_ID => Some("string"),
    _ => None,
  }
}

impl JavascriptParserPlugin for ModernModulePlugin {
  fn evaluate_typeof(
    &self,
    parser: &mut JavascriptParser,
    expr: &UnaryExpr,
    for_name: &str,
  ) -> Option<BasicEvaluatedExpression> {
    if for_name == WEBPACK_LAYER {
      let value = if parser.module_layer.is_none() {
        "object"
      } else {
        "string"
      };
      Some(eval::evaluate_to_string(
        value.to_string(),
        expr.span.real_lo(),
        expr.span.real_hi(),
      ))
    } else {
      get_typeof_evaluate_of_api(for_name).map(|res| {
        eval::evaluate_to_string(res.to_string(), expr.span.real_lo(), expr.span.real_hi())
      })
    }
  }

  fn identifier(
    &self,
    _parser: &mut JavascriptParser,
    _ident: &Ident,
    for_name: &str,
  ) -> Option<bool> {
    match for_name {
      WEBPACK_REQUIRE => Some(true),
      WEBPACK_HASH => Some(true),
      WEBPACK_LAYER => Some(true),
      WEBPACK_PUBLIC_PATH => Some(true),
      WEBPACK_MODULES => Some(true),
      WEBPACK_CHUNK_LOAD => Some(true),
      WEBPACK_MODULE => Some(true),
      WEBPACK_BASE_URI => Some(true),
      NON_WEBPACK_REQUIRE => Some(true),
      SYSTEM_CONTEXT => Some(true),
      WEBPACK_SHARE_SCOPES => Some(true),
      WEBPACK_INIT_SHARING => Some(true),
      WEBPACK_NONCE => Some(true),
      WEBPACK_CHUNK_NAME => Some(true),
      WEBPACK_RUNTIME_ID => Some(true),
      WEBPACK_GET_SCRIPT_FILENAME => Some(true),
      // rspack specific
      RSPACK_VERSION => Some(true),
      RSPACK_UNIQUE_ID => Some(true),
      _ => None,
    }
  }

  fn evaluate_identifier(
    &self,
    parser: &mut JavascriptParser,
    ident: &str,
    start: u32,
    end: u32,
  ) -> Option<eval::BasicEvaluatedExpression> {
    if ident == WEBPACK_LAYER {
      if let Some(layer) = parser.module_layer {
        Some(eval::evaluate_to_string(layer.into(), start, end))
      } else {
        Some(eval::evaluate_to_null(start, end))
      }
    } else {
      None
    }
  }

  fn member(
    &self,
    parser: &mut JavascriptParser,
    member_expr: &swc_core::ecma::ast::MemberExpr,
    _name: &str,
  ) -> Option<bool> {
    macro_rules! not_supported_expr {
      ($check: ident, $expr: ident, $name: literal) => {
        if expr_matcher::$check($expr) {
          return Some(true);
        }
      };
    }

    let expr = &swc_core::ecma::ast::Expr::Member(member_expr.to_owned());

    if let Some(root) = extract_member_root(expr)
      && let s = root.sym.as_str()
      && parser.is_unresolved_ident(s)
    {
      if s == "require" {
        not_supported_expr!(is_require_extensions, expr, "require.extensions");
        not_supported_expr!(is_require_config, expr, "require.config");
        not_supported_expr!(is_require_version, expr, "require.version");
        not_supported_expr!(is_require_amd, expr, "require.amd");
        not_supported_expr!(is_require_include, expr, "require.include");
        not_supported_expr!(is_require_onerror, expr, "require.onError");
        not_supported_expr!(is_require_main_require, expr, "require.main.require");
      } else if s == "module" {
        not_supported_expr!(is_module_parent_require, expr, "module.parent.require");
      }
    }

    if expr_matcher::is_require_cache(expr)
      || expr_matcher::is_require_main(expr)
      || expr_matcher::is_webpack_module_id(expr)
    {
      Some(true)
    } else {
      None
    }
  }

  fn call(&self, parser: &mut JavascriptParser, call_expr: &CallExpr, _name: &str) -> Option<bool> {
    macro_rules! not_supported_call {
      ($check: ident, $name: literal) => {
        if let Callee::Expr(box Expr::Member(expr)) = &call_expr.callee
          && expr_matcher::$check(&Expr::Member(expr.to_owned()))
        {
          return Some(true);
        }
      };
    }

    let root = call_expr
      .callee
      .as_expr()
      .and_then(|expr| extract_member_root(expr));

    if let Some(root) = root
      && let s = root.sym.as_str()
      && parser.is_unresolved_ident(s)
    {
      if s == "require" {
        not_supported_call!(is_require_config, "require.config()");
        not_supported_call!(is_require_include, "require.include()");
        not_supported_call!(is_require_onerror, "require.onError()");
        not_supported_call!(is_require_main_require, "require.main.require()");
      } else if s == "module" {
        not_supported_call!(is_module_parent_require, "module.parent.require()");
      }
    }

    None
  }
}
