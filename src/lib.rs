// CREATURE Framework Library
// Expose modules for use in binaries and tests

pub mod api;
pub mod interface;
pub mod models;
pub mod systems;
pub mod utils;
pub mod thought_viewer;

// Re-export commonly used types
pub use models::types::{Thought, DimensionalPosition};
pub use thought_viewer::ThoughtViewer;
