use rspack_cacheable::{cacheable, cacheable_dyn, with::AsPreset};
use rspack_core::{
  AffectType, AsContextDependency, AsDependencyTemplate, Dependency, DependencyCategory,
  DependencyId, DependencyType, ModuleDependency, RealDependencyLocation,
};
use rspack_util::atom::Atom;

#[cacheable]
#[derive(Debug, Clone)]
pub struct RequireEnsureItemDependency {
  id: DependencyId,
  #[cacheable(with=AsPreset)]
  request: Atom,
  range: RealDependencyLocation,
}

impl RequireEnsureItemDependency {
  pub fn new(request: Atom, range: RealDependencyLocation) -> Self {
    Self {
      id: DependencyId::new(),
      request,
      range,
    }
  }
}

#[cacheable_dyn]
impl Dependency for RequireEnsureItemDependency {
  fn id(&self) -> &rspack_core::DependencyId {
    &self.id
  }

  fn category(&self) -> &DependencyCategory {
    &DependencyCategory::CommonJS
  }

  fn dependency_type(&self) -> &DependencyType {
    &DependencyType::RequireEnsureItem
  }

  fn range(&self) -> Option<&RealDependencyLocation> {
    Some(&self.range)
  }

  fn could_affect_referencing_module(&self) -> AffectType {
    AffectType::True
  }
}

#[cacheable_dyn]
impl ModuleDependency for RequireEnsureItemDependency {
  fn request(&self) -> &str {
    &self.request
  }
}

impl AsDependencyTemplate for RequireEnsureItemDependency {}

impl AsContextDependency for RequireEnsureItemDependency {}
