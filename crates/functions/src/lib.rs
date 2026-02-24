// Functions crate - Feature-based modules
pub mod features;

// Re-export configure functions for backward compatibility
pub use features::{
    configure_bootstrap, configure_dinners, configure_health, configure_profiles, configure_roles, configure_user_roles,
    configure_users,
};
