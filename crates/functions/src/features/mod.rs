pub mod health;
pub mod profiles;
pub mod roles;
pub mod user_roles;
pub mod users;

// Re-export configure functions for easy use in main app
pub use health::configure_health;
pub use profiles::configure_profiles;
pub use roles::configure_roles;
pub use user_roles::configure_user_roles;
pub use users::configure_users;
