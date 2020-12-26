use crate::stdlib::boxed::Box;
use crate::{Engine, EvalAltResult, Module, ModuleResolver, Position, Shared};

/// Empty/disabled module resolution service that acts as a dummy.
///
/// # Example
///
/// ```
/// use rhai::{Engine, Module};
/// use rhai::module_resolvers::DummyModuleResolver;
///
/// let resolver = DummyModuleResolver::new();
/// let mut engine = Engine::new();
/// engine.set_module_resolver(resolver);
/// ```
#[derive(Debug, Copy, Eq, PartialEq, Clone, Default, Hash)]
pub struct DummyModuleResolver;

impl DummyModuleResolver {
    /// Create a new [`DummyModuleResolver`].
    ///
    /// # Example
    ///
    /// ```
    /// use rhai::{Engine, Module};
    /// use rhai::module_resolvers::DummyModuleResolver;
    ///
    /// let resolver = DummyModuleResolver::new();
    /// let mut engine = Engine::new();
    /// engine.set_module_resolver(resolver);
    /// ```
    #[inline(always)]
    pub fn new() -> Self {
        Default::default()
    }
}

impl ModuleResolver for DummyModuleResolver {
    #[inline(always)]
    fn resolve(
        &self,
        _: &Engine,
        path: &str,
        pos: Position,
    ) -> Result<Shared<Module>, Box<EvalAltResult>> {
        EvalAltResult::ErrorModuleNotFound(path.into(), pos).into()
    }
}
