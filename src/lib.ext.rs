//! The module's extension (hand-written; ADR-0031).
//!
//! Everything a module adds beside its generated services lives here, never in `lib.rs`, which
//! the generator overwrites whole. The generated module and builder carry the two types below
//! (`ModuleExt`, `ModuleBuilderExt`) and dereference to them, so a field declared here reads as
//! `module.field` / `builder.field`. Methods are ordinary `impl` blocks on the generated types.
//!
//! Replace the example with the module's own state: a hand-written write service built from
//! the generated parts, an optional port a host may set, a seam.

use crate::ModuleParts;

/// State the module adds; read through the generated module's `Deref`.
pub struct ModuleExt {
    /// Example: a value a host sets on the builder and the module reads back.
    pub(crate) greeting: String,
}

/// State the builder adds; `Default` is the builder's starting state.
#[derive(Default)]
pub struct ModuleBuilderExt {
    pub(crate) greeting: Option<String>,
}

impl ModuleBuilderExt {
    /// The build hook: called by the generated `build()` once the pool and every generated
    /// service exist (`parts`), to build the extension's state from them.
    pub(crate) fn build(self, _parts: &ModuleParts<'_>) -> anyhow::Result<ModuleExt> {
        Ok(ModuleExt { greeting: self.greeting.unwrap_or_else(|| "hello".to_string()) })
    }
}

impl crate::ThisModuleBuilder {
    /// Example builder method: an option a host sets before `build()`.
    pub fn with_greeting(mut self, greeting: impl Into<String>) -> Self {
        self.greeting = Some(greeting.into());
        self
    }
}

impl crate::ThisModule {
    /// Example module method over the extension's state.
    pub fn greeting(&self) -> &str {
        &self.greeting
    }
}
