// Database models - SeaORM entities
// These represent database tables

pub mod attendance;
pub mod cell_group;
pub mod dinner;
pub mod dinner_participant;
pub mod family_relationship;
pub mod giving;
pub mod membership_history;
pub mod ministry;
pub mod role;
pub mod spiritual_milestone;
pub mod user;
pub mod user_address;
pub mod user_membership;
pub mod user_ministry;
pub mod user_profile;
pub mod user_role;
pub mod user_skill;
pub mod visitable_family;
pub mod visit_assignment;
pub mod zone;

// Re-export for convenience
pub use attendance::{
    ActiveModel as AttendanceActiveModel, Entity as Attendance, Model as AttendanceModel,
};
pub use cell_group::{
    ActiveModel as CellGroupActiveModel, Entity as CellGroup, Model as CellGroupModel,
};
pub use dinner::{ActiveModel as DinnerActiveModel, Entity as Dinner, Model as DinnerModel};
pub use dinner_participant::{
    ActiveModel as DinnerParticipantActiveModel, Entity as DinnerParticipant,
    Model as DinnerParticipantModel,
};
pub use family_relationship::{
    ActiveModel as FamilyRelationshipActiveModel, Entity as FamilyRelationship,
    Model as FamilyRelationshipModel,
};
pub use giving::{ActiveModel as GivingActiveModel, Entity as Giving, Model as GivingModel};
pub use membership_history::{
    ActiveModel as MembershipHistoryActiveModel, Entity as MembershipHistory,
    Model as MembershipHistoryModel,
};
pub use ministry::{
    ActiveModel as MinistryActiveModel, Entity as Ministry, Model as MinistryModel,
};
pub use role::{ActiveModel as RoleActiveModel, Entity as Role, Model as RoleModel};
pub use spiritual_milestone::{
    ActiveModel as SpiritualMilestoneActiveModel, Entity as SpiritualMilestone,
    Model as SpiritualMilestoneModel,
};
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
pub use user_skill::{
    ActiveModel as UserSkillActiveModel, Entity as UserSkill, Model as UserSkillModel,
};
pub use visitable_family::{
    ActiveModel as VisitableFamilyActiveModel, Entity as VisitableFamily,
    Model as VisitableFamilyModel,
};
pub use visit_assignment::{
    ActiveModel as VisitAssignmentActiveModel, Entity as VisitAssignment,
    Model as VisitAssignmentModel,
};
pub use zone::{ActiveModel as ZoneActiveModel, Entity as Zone, Model as ZoneModel};
