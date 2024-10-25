use rspack_cacheable::{cacheable, cacheable_dyn, with::AsPreset};
use rspack_core::{
  module_id, AsContextDependency, Compilation, Dependency, DependencyCategory, DependencyId,
  DependencyTemplate, DependencyType, ModuleDependency, RealDependencyLocation, RuntimeSpec,
  TemplateContext, TemplateReplaceSource,
};
use swc_core::ecma::atoms::Atom;

#[cacheable]
#[derive(Debug, Clone)]
pub struct ModuleHotDeclineDependency {
  id: DependencyId,
  #[cacheable(with=AsPreset)]
  request: Atom,
  range: RealDependencyLocation,
}

impl ModuleHotDeclineDependency {
  pub fn new(request: Atom, range: RealDependencyLocation) -> Self {
    Self {
      id: DependencyId::new(),
      request,
      range,
    }
  }
}

#[cacheable_dyn]
impl Dependency for ModuleHotDeclineDependency {
  fn id(&self) -> &DependencyId {
    &self.id
  }

  fn category(&self) -> &DependencyCategory {
    &DependencyCategory::CommonJS
  }

  fn dependency_type(&self) -> &DependencyType {
    &DependencyType::ModuleHotDecline
  }

  fn range(&self) -> Option<&RealDependencyLocation> {
    Some(&self.range)
  }

  fn could_affect_referencing_module(&self) -> rspack_core::AffectType {
    rspack_core::AffectType::True
  }
}

#[cacheable_dyn]
impl ModuleDependency for ModuleHotDeclineDependency {
  fn request(&self) -> &str {
    &self.request
  }

  fn user_request(&self) -> &str {
    &self.request
  }

  fn set_request(&mut self, request: String) {
    self.request = request.into();
  }

  fn weak(&self) -> bool {
    true
  }
}

#[cacheable_dyn]
impl DependencyTemplate for ModuleHotDeclineDependency {
  fn apply(
    &self,
    source: &mut TemplateReplaceSource,
    code_generatable_context: &mut TemplateContext,
  ) {
    source.replace(
      self.range.start,
      self.range.end,
      module_id(
        code_generatable_context.compilation,
        &self.id,
        &self.request,
        self.weak(),
      )
      .as_str(),
      None,
    );
  }

  fn dependency_id(&self) -> Option<DependencyId> {
    Some(self.id)
  }

  fn update_hash(
    &self,
    _hasher: &mut dyn std::hash::Hasher,
    _compilation: &Compilation,
    _runtime: Option<&RuntimeSpec>,
  ) {
  }
}

impl AsContextDependency for ModuleHotDeclineDependency {}
