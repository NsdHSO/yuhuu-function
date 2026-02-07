pub mod health;
pub mod profiles;
pub mod users;

// Re-export configure functions for easy use in main app
pub use health::configure_health;
pub use profiles::configure_profiles;
pub use users::configure_users;
