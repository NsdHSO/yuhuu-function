// Models crate - Church Management System
// Separated into dto (database) models and internal (API request/response)
// Following SOLID principles with normalized tables

// Database models (SeaORM entities)
pub mod dto;

// API request/response types
pub mod internal;

// Re-export database models for convenience
pub use dto::{
    Attendance, AttendanceActiveModel, AttendanceModel, CellGroup, CellGroupActiveModel,
    CellGroupModel, Giving, GivingActiveModel, GivingModel, Ministry, MinistryActiveModel,
    MinistryModel, Role, RoleActiveModel, RoleModel, User, UserActiveModel, UserAddress,
    UserAddressActiveModel, UserAddressModel, UserMembership, UserMembershipActiveModel,
    UserMembershipModel, UserMinistry, UserMinistryActiveModel, UserMinistryModel, UserModel,
    UserProfile, UserProfileActiveModel, UserProfileModel, UserRole, UserRoleActiveModel,
    UserRoleModel, Zone, ZoneActiveModel, ZoneModel,
};
