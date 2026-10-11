//! The extension of `example_service.rs` (hand-written; ADR-0031).
//!
//! The generated service is an alias of the framework's `GenericCrudService`, a type from
//! another crate, so its extra operations are an extension trait: callers bring the trait into
//! scope (`use crate::application::service::ExampleServiceExt;`) and call it on the service.

/// Operations the module adds to the generated `ExampleService`.
pub trait ExampleServiceExt {
    /// Example: a name for the records this service holds.
    fn record_kind(&self) -> &'static str;
}

impl ExampleServiceExt for super::ExampleService {
    fn record_kind(&self) -> &'static str {
        "example"
    }
}
