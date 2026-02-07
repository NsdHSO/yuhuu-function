// Functions crate - Feature-based modules
pub mod features;

// Re-export configure functions for backward compatibility
pub use features::{configure_health, configure_profiles, configure_users};
