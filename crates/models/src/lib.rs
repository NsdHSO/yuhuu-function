// Models crate - Church Management System
// Data models for users, zones, cell groups, ministries, attendance, and giving
// Following SOLID principles with normalized tables

// Core user tables (normalized)
pub mod user;
pub mod user_profile;
pub mod user_address;
pub mod user_membership;

// Church structure
pub mod zone;
pub mod cell_group;

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
pub use user::{Entity as User, Model as UserModel, ActiveModel as UserActiveModel};
pub use user_profile::{Entity as UserProfile, Model as UserProfileModel, ActiveModel as UserProfileActiveModel};
pub use user_address::{Entity as UserAddress, Model as UserAddressModel, ActiveModel as UserAddressActiveModel};
pub use user_membership::{Entity as UserMembership, Model as UserMembershipModel, ActiveModel as UserMembershipActiveModel};
pub use zone::{Entity as Zone, Model as ZoneModel, ActiveModel as ZoneActiveModel};
pub use cell_group::{Entity as CellGroup, Model as CellGroupModel, ActiveModel as CellGroupActiveModel};
pub use role::{Entity as Role, Model as RoleModel, ActiveModel as RoleActiveModel};
pub use user_role::{Entity as UserRole, Model as UserRoleModel, ActiveModel as UserRoleActiveModel};
pub use ministry::{Entity as Ministry, Model as MinistryModel, ActiveModel as MinistryActiveModel};
pub use user_ministry::{Entity as UserMinistry, Model as UserMinistryModel, ActiveModel as UserMinistryActiveModel};
pub use attendance::{Entity as Attendance, Model as AttendanceModel, ActiveModel as AttendanceActiveModel};
pub use giving::{Entity as Giving, Model as GivingModel, ActiveModel as GivingActiveModel};
