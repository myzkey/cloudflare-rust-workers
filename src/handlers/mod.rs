pub mod bar;
pub mod baz;
pub mod foo;

// Re-export all handlers for easy access
pub use bar::handle_post;
pub use baz::handle_delete;
pub use foo::handle_get;
