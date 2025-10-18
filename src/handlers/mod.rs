pub mod foo;
pub mod bar;
pub mod baz;

// Re-export all handlers for easy access
pub use foo::handle_get;
pub use bar::handle_post;
pub use baz::handle_delete;