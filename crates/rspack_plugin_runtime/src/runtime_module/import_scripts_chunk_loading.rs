use cow_utils::CowUtils;
use rspack_collections::Identifier;
use rspack_core::{
  compile_boolean_matcher, impl_runtime_module, BooleanMatcher, Chunk, ChunkUkey, Compilation,
  RuntimeGlobals, RuntimeModule, RuntimeModuleStage,
};

use super::{
  generate_javascript_hmr_runtime,
  utils::{chunk_has_js, get_output_dir},
};
use crate::{
  get_chunk_runtime_requirements,
  runtime_module::utils::{get_initial_chunk_ids, stringify_chunks},
};

#[impl_runtime_module]
#[derive(Debug, Default)]
pub struct ImportScriptsChunkLoadingRuntimeModule {
  id: Identifier,
  chunk: Option<ChunkUkey>,
  with_create_script_url: bool,
}

impl ImportScriptsChunkLoadingRuntimeModule {
  pub fn new(with_create_script_url: bool) -> Self {
    Self::with_default(
      Identifier::from("webpack/runtime/import_scripts_chunk_loading"),
      None,
      with_create_script_url,
    )
  }

  fn generate_base_uri(
    &self,
    chunk: &Chunk,
    compilation: &Compilation,
  ) -> rspack_error::Result<String> {
    let base_uri = if let Some(base_uri) = chunk
      .get_entry_options(&compilation.chunk_group_by_ukey)
      .and_then(|options| options.base_uri.as_ref())
      .and_then(|base_uri| serde_json::to_string(base_uri).ok())
    {
      base_uri
    } else {
      let root_output_dir = get_output_dir(chunk, compilation, false)?;
      format!(
        "self.location + {}",
        serde_json::to_string(&if root_output_dir.is_empty() {
          "".to_string()
        } else {
          format!("/../{root_output_dir}")
        })
        .expect("should able to be serde_json::to_string")
      )
    };
    Ok(format!("{} = {};\n", RuntimeGlobals::BASE_URI, base_uri))
  }

  fn generate(&self, compilation: &Compilation) -> rspack_error::Result<String> {
    let chunk = compilation
      .chunk_by_ukey
      .expect_get(&self.chunk.expect("The chunk should be attached."));

    let runtime_requirements = get_chunk_runtime_requirements(compilation, &chunk.ukey);
    let initial_chunks = get_initial_chunk_ids(self.chunk, compilation, chunk_has_js);

    let with_base_uri = runtime_requirements.contains(RuntimeGlobals::BASE_URI);
    let with_hmr = runtime_requirements.contains(RuntimeGlobals::HMR_DOWNLOAD_UPDATE_HANDLERS);
    let with_hmr_manifest = runtime_requirements.contains(RuntimeGlobals::HMR_DOWNLOAD_MANIFEST);
    let with_loading = runtime_requirements.contains(RuntimeGlobals::ENSURE_CHUNK_HANDLERS);

    let condition_map =
      compilation
        .chunk_graph
        .get_chunk_condition_map(&chunk.ukey, compilation, chunk_has_js);
    let has_js_matcher = compile_boolean_matcher(&condition_map);

    let mut generated_code = String::new();

    if with_base_uri {
      generated_code.push_str(&self.generate_base_uri(chunk, compilation)?);
    }

    // object to store loaded chunks
    // "1" means "already loaded"
    if with_hmr {
      let state_expression = format!("{}_importScripts", RuntimeGlobals::HMR_RUNTIME_STATE_PREFIX);
      generated_code.push_str(&format!(
        "var installedChunks = {} = {} || {};\n",
        state_expression,
        state_expression,
        &stringify_chunks(&initial_chunks, 1)
      ));
    } else {
      generated_code.push_str(&format!(
        "var installedChunks = {};\n",
        &stringify_chunks(&initial_chunks, 1)
      ));
    }

    if with_loading {
      let chunk_loading_global_expr = format!(
        "{}[\"{}\"]",
        &compilation.options.output.global_object, &compilation.options.output.chunk_loading_global
      );

      let body = if matches!(has_js_matcher, BooleanMatcher::Condition(false)) {
        "installedChunks[chunkId] = 1;".to_string()
      } else {
        let url = if self.with_create_script_url {
          format!(
            "{}({} + {}(chunkId))",
            RuntimeGlobals::CREATE_SCRIPT_URL,
            RuntimeGlobals::PUBLIC_PATH,
            RuntimeGlobals::GET_CHUNK_SCRIPT_FILENAME
          )
        } else {
          format!(
            "{} + {}(chunkId)",
            RuntimeGlobals::PUBLIC_PATH,
            RuntimeGlobals::GET_CHUNK_SCRIPT_FILENAME
          )
        };
        format!(
          r#"
          // "1" is the signal for "already loaded
          if (!installedChunks[chunkId]) {{
            if ({}) {{
              importScripts({});
            }}
          }}
          "#,
          &has_js_matcher.render("chunkId"),
          url
        )
      };

      // If chunkId not corresponding chunkName will skip load it.
      generated_code.push_str(
        &include_str!("runtime/import_scripts_chunk_loading.js")
          .cow_replace("$BODY$", body.as_str())
          .cow_replace("$CHUNK_LOADING_GLOBAL_EXPR$", &chunk_loading_global_expr),
      );
    }

    if with_hmr {
      let url = if self.with_create_script_url {
        format!(
          "{}({} + {}(chunkId))",
          RuntimeGlobals::CREATE_SCRIPT_URL,
          RuntimeGlobals::PUBLIC_PATH,
          RuntimeGlobals::GET_CHUNK_UPDATE_SCRIPT_FILENAME
        )
      } else {
        format!(
          "{} + {}(chunkId)",
          RuntimeGlobals::PUBLIC_PATH,
          RuntimeGlobals::GET_CHUNK_UPDATE_SCRIPT_FILENAME
        )
      };
      generated_code.push_str(
        &include_str!("runtime/import_scripts_chunk_loading_with_hmr.js")
          .cow_replace("$URL$", &url)
          .cow_replace("$globalObject$", &compilation.options.output.global_object)
          .cow_replace(
            "$hotUpdateGlobal$",
            &serde_json::to_string(&compilation.options.output.hot_update_global)
              .expect("failed to serde_json::to_string(hot_update_global)"),
          ),
      );
      generated_code.push_str(&generate_javascript_hmr_runtime("importScripts"));
    }

    if with_hmr_manifest {
      // TODO: import_scripts_chunk_loading_with_hmr_manifest same as jsonp_chunk_loading_with_hmr_manifest
      generated_code.push_str(&include_str!(
        "runtime/import_scripts_chunk_loading_with_hmr_manifest.js"
      ));
    }

    Ok(generated_code)
  }
}

impl RuntimeModule for ImportScriptsChunkLoadingRuntimeModule {
  fn name(&self) -> Identifier {
    self.id
  }

  fn attach(&mut self, chunk: ChunkUkey) {
    self.chunk = Some(chunk);
  }

  fn stage(&self) -> RuntimeModuleStage {
    RuntimeModuleStage::Attach
  }
}
