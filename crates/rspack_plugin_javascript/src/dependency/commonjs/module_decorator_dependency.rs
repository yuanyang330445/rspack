use rspack_cacheable::{cacheable, cacheable_dyn};
use rspack_core::{
  create_exports_object_referenced, create_no_exports_referenced, AsContextDependency, Compilation,
  Dependency, DependencyId, DependencyTemplate, DependencyType, InitFragmentKey, InitFragmentStage,
  ModuleDependency, NormalInitFragment, RuntimeGlobals, RuntimeSpec, TemplateContext,
  TemplateReplaceSource,
};
use rspack_util::ext::DynHash;

#[cacheable]
#[derive(Debug, Clone)]
pub struct ModuleDecoratorDependency {
  decorator: RuntimeGlobals,
  allow_exports_access: bool,
  id: DependencyId,
}

impl ModuleDecoratorDependency {
  pub fn new(decorator: RuntimeGlobals, allow_exports_access: bool) -> Self {
    Self {
      decorator,
      allow_exports_access,
      id: DependencyId::new(),
    }
  }
}

#[cacheable_dyn]
impl ModuleDependency for ModuleDecoratorDependency {
  fn request(&self) -> &str {
    "self"
  }
}

#[cacheable_dyn]
impl DependencyTemplate for ModuleDecoratorDependency {
  fn apply(
    &self,
    _source: &mut TemplateReplaceSource,
    code_generatable_context: &mut TemplateContext,
  ) {
    let TemplateContext {
      runtime_requirements,
      init_fragments,
      compilation,
      module,
      ..
    } = code_generatable_context;

    runtime_requirements.insert(RuntimeGlobals::MODULE_LOADED);
    runtime_requirements.insert(RuntimeGlobals::MODULE_ID);
    runtime_requirements.insert(RuntimeGlobals::MODULE);
    runtime_requirements.insert(self.decorator);

    let module_graph = compilation.get_module_graph();
    let module = module_graph
      .module_by_identifier(&module.identifier())
      .expect("should have mgm");
    let module_argument = module.get_module_argument();

    // ref: tests/webpack-test/cases/scope-hoisting/issue-5096 will return a `null` as module id
    let module_id = compilation
      .chunk_graph
      .get_module_id(module.identifier())
      .map(|s| s.to_string())
      .unwrap_or_default();

    init_fragments.push(Box::new(NormalInitFragment::new(
      format!(
        "/* module decorator */ {} = {}({});\n",
        module_argument,
        self.decorator.name(),
        module_argument
      ),
      InitFragmentStage::StageProvides,
      0,
      InitFragmentKey::ModuleDecorator(module_id),
      None,
    )));
  }

  fn dependency_id(&self) -> Option<rspack_core::DependencyId> {
    None
  }

  fn update_hash(
    &self,
    hasher: &mut dyn std::hash::Hasher,
    _compilation: &Compilation,
    _runtime: Option<&RuntimeSpec>,
  ) {
    self.decorator.dyn_hash(hasher);
    self.allow_exports_access.dyn_hash(hasher);
  }
}

impl AsContextDependency for ModuleDecoratorDependency {}

#[cacheable_dyn]
impl Dependency for ModuleDecoratorDependency {
  fn id(&self) -> &DependencyId {
    &self.id
  }

  fn resource_identifier(&self) -> Option<&str> {
    Some("self")
  }

  fn dependency_type(&self) -> &DependencyType {
    &DependencyType::ModuleDecorator
  }

  fn get_referenced_exports(
    &self,
    _module_graph: &rspack_core::ModuleGraph,
    _runtime: Option<&rspack_core::RuntimeSpec>,
  ) -> Vec<rspack_core::ExtendedReferencedExport> {
    if self.allow_exports_access {
      create_exports_object_referenced()
    } else {
      create_no_exports_referenced()
    }
  }

  fn could_affect_referencing_module(&self) -> rspack_core::AffectType {
    rspack_core::AffectType::False
  }
}
