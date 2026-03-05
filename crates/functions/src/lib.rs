// Functions crate - Feature-based modules
pub mod features;

// Re-export configure functions for backward compatibility
pub use features::{
    configure_admin, configure_bootstrap, configure_dinners, configure_family_relationships,
    configure_health, configure_membership_history, configure_profiles, configure_roles,
    configure_spiritual_milestones, configure_user_roles, configure_user_skills,
    configure_users,
};
