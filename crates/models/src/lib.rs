// Models crate - Church Management System
// Data models for users, zones, cell groups, ministries, attendance, and giving
// Following SOLID principles with normalized tables

// Core user tables (normalized)
pub mod user;
pub mod user_address;
pub mod user_membership;
pub mod user_profile;

// Church structure
pub mod cell_group;
pub mod zone;

// Roles and permissions
pub mod role;
pub mod user_role;

// Ministry involvement
pub mod ministry;
pub mod user_ministry;

// Tracking
pub mod attendance;
pub mod giving;

// Re-export commonly used types
pub use attendance::{
    ActiveModel as AttendanceActiveModel, Entity as Attendance, Model as AttendanceModel,
};
pub use cell_group::{
    ActiveModel as CellGroupActiveModel, Entity as CellGroup, Model as CellGroupModel,
};
pub use giving::{ActiveModel as GivingActiveModel, Entity as Giving, Model as GivingModel};
pub use ministry::{
    ActiveModel as MinistryActiveModel, Entity as Ministry, Model as MinistryModel,
};
pub use role::{ActiveModel as RoleActiveModel, Entity as Role, Model as RoleModel};
pub use user::{ActiveModel as UserActiveModel, Entity as User, Model as UserModel};
pub use user_address::{
    ActiveModel as UserAddressActiveModel, Entity as UserAddress, Model as UserAddressModel,
};
pub use user_membership::{
    ActiveModel as UserMembershipActiveModel, Entity as UserMembership,
    Model as UserMembershipModel,
};
pub use user_ministry::{
    ActiveModel as UserMinistryActiveModel, Entity as UserMinistry, Model as UserMinistryModel,
};
pub use user_profile::{
    ActiveModel as UserProfileActiveModel, Entity as UserProfile, Model as UserProfileModel,
};
pub use user_role::{
    ActiveModel as UserRoleActiveModel, Entity as UserRole, Model as UserRoleModel,
};
pub use zone::{ActiveModel as ZoneActiveModel, Entity as Zone, Model as ZoneModel};
