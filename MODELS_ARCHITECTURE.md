# Models Architecture

## ✅ Reorganization Complete

The models crate has been reorganized to separate concerns:

### 📁 Directory Structure

```
crates/models/src/
├── lib.rs                          # Main module exports
├── internal/                       # Database models (SeaORM entities)
│   ├── mod.rs                      # Internal module exports
│   ├── user.rs                     # User entity (bridge to auth)
│   ├── user_profile.rs             # User profile entity
│   ├── user_address.rs             # User address entity
│   ├── user_membership.rs          # Church membership entity
│   ├── user_role.rs                # User role assignments
│   ├── user_ministry.rs            # Ministry participation
│   ├── attendance.rs               # Attendance tracking
│   ├── giving.rs                   # Giving/tithes tracking
│   ├── role.rs                     # Role definitions
│   ├── ministry.rs                 # Ministry definitions
│   ├── zone.rs                     # Church zones
│   └── cell_group.rs               # Cell groups
└── dto/                            # Data Transfer Objects
    ├── mod.rs                      # DTO module exports
    ├── user.rs                     # User DTOs (request/response)
    └── profile.rs                  # Profile DTOs (request/response)
```

## 🎯 Design Principles

### Internal Models (`models::internal`)

**Purpose:** Database entities managed by SeaORM
**When to use:** Data layer, database queries, migrations

**Characteristics:**
- ✅ Use SeaORM derives (`DeriveEntityModel`, etc.)
- ✅ Use database types (`NaiveDate`, `NaiveDateTime`, `Decimal`)
- ✅ Include all database fields (including `uuid`, timestamps)
- ✅ Define relationships (foreign keys, relations)

**Example:**
```rust
use models::internal::{User, UserProfile, user};

// Query database
let user = User::find_by_id(1).one(&db).await?;

// Use column enums
User::find()
    .filter(user::Column::AuthUserId.eq("uuid-123"))
    .one(&db)
    .await?;
```

### DTOs (`models::dto`)

**Purpose:** API request/response types
**When to use:** HTTP handlers, external APIs, JSON serialization

**Characteristics:**
- ✅ Use Serde derives (`Serialize`, `Deserialize`)
- ✅ Use string types for dates (client-friendly: "YYYY-MM-DD")
- ✅ Hide internal fields (no `uuid`, no sensitive data)
- ✅ Flattened/transformed for API convenience
- ✅ Include validation helpers

**Example:**
```rust
use models::dto::{CreateProfileRequest, ProfileResponse, UserResponse};

// In HTTP handler
pub async fn create_profile(
    body: web::Json<CreateProfileRequest>,  // ← DTO for request
) -> Result<HttpResponse> {
    // ... create dto model from DTO
    let profile = /* database insert */;

    let response: ProfileResponse = profile.into();  // ← Convert to DTO
    Ok(HttpResponse::Created().json(response))
}
```

## 📊 Current DTOs

### User DTOs (`models::dto::user`)

**LinkUserRequest**
```rust
{
  "email": "user@example.com"
}
```

**ListUsersQuery**
```rust
?page=1&limit=20
```

**UserResponse**
```rust
{
  "id": 1,
  "auth_user_id": "uuid",
  "email": "user@example.com",
  "full_name": "John Doe",
  "role": "Member",
  "is_email_verified": true,
  "created_at": "2026-02-06T12:00:00",
  "updated_at": "2026-02-06T12:00:00"
}
```

### Profile DTOs (`models::dto::profile`)

**CreateProfileRequest**
```rust
{
  "phone": "+1234567890",
  "date_of_birth": "1990-01-15",  // String, not NaiveDate
  "gender": "Male",
  "occupation": "Engineer",
  ...
}
```

**UpdateProfileRequest** (partial)
```rust
{
  "phone": "+9876543210",         // Only update specific fields
  "occupation": "Senior Engineer"
}
```

**ProfileResponse**
```rust
{
  "id": 1,
  "user_id": 1,
  "phone": "+1234567890",
  "date_of_birth": "1990-01-15",  // Formatted as string
  ...
}
```

## 🔄 Conversions

### Internal → DTO

Implement `From<InternalModel> for DTO`:

```rust
impl From<UserProfileModel> for ProfileResponse {
    fn from(profile: UserProfileModel) -> Self {
        ProfileResponse {
            id: profile.id,
            user_id: profile.user_id,
            date_of_birth: profile.date_of_birth.map(|d| d.to_string()),  // Convert NaiveDate → String
            ...
        }
    }
}
```

### DTO → Internal

Manual conversion in handlers:

```rust
pub async fn create_profile(
    body: web::Json<CreateProfileRequest>,
) -> Result<HttpResponse> {
    // Parse string date to NaiveDate
    let date_of_birth = match &body.date_of_birth {
        Some(date_str) => NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok(),
        None => None,
    };

    // Create dto model
    let new_profile = user_profile::ActiveModel {
        phone: Set(body.phone.clone()),
        date_of_birth: Set(date_of_birth),
        ...
    };

    let result = new_profile.insert(&db).await?;
    Ok(HttpResponse::Created().json(ProfileResponse::from(result)))
}
```

## 📝 Import Guidelines

### In Handlers (`crates/functions/src/handlers/*`)

```rust
// Import DTOs for API layer
use models::dto::{
    CreateProfileRequest,
    UpdateProfileRequest,
    ProfileResponse,
    UserResponse,
};

// Import dto models for database operations
use models::internal::{
    UserProfile,
    user_profile,  // For Column enums
};
```

### In Services (`crates/auth-integration/*`)

```rust
// Import dto models only
use models::internal::{User, UserModel, user};
```

### In Migrations (`migration/*`)

```rust
// Use dto module paths directly
// Or generate from SeaORM entities
```

## 🎨 Benefits of This Architecture

### ✅ **Separation of Concerns**
- Database layer (internal) doesn't leak to API
- API layer (dto) doesn't depend on database structure

### ✅ **Type Safety**
- Compile-time validation of request/response shapes
- Clear contracts between layers

### ✅ **Flexibility**
- Can change database schema without breaking API
- Can change API format without touching database
- Easy to add validation, transformations

### ✅ **Documentation**
- DTOs serve as API documentation
- Internal models document database schema

### ✅ **Testability**
- Can mock DTOs for API tests
- Can mock internal models for integration tests

## 🚀 Adding New DTOs

### 1. Create DTO file in `models/src/dto/`

```rust
// models/src/internal/address.rs
use serde::{Deserialize, Serialize};
use crate::internal::UserAddressModel;

#[derive(Debug, Deserialize)]
pub struct CreateAddressRequest {
    pub address_line1: String,
    pub city: String,
    ...
}

#[derive(Debug, Serialize)]
pub struct AddressResponse {
    pub id: i64,
    pub user_id: i64,
    pub address_line1: String,
    ...
}

impl From<UserAddressModel> for AddressResponse {
    fn from(addr: UserAddressModel) -> Self {
        AddressResponse {
            id: addr.id,
            user_id: addr.user_id,
            address_line1: addr.address_line1.unwrap_or_default(),
            ...
        }
    }
}
```

### 2. Export in `models/src/dto/mod.rs`

```rust
pub mod address;
pub use address::*;
```

### 3. Use in handlers

```rust
use models::dto::{CreateAddressRequest, AddressResponse};
use models::internal::{UserAddress, user_address};

pub async fn create_address(
    body: web::Json<CreateAddressRequest>,
) -> Result<HttpResponse> {
    let new_addr = user_address::ActiveModel {
        address_line1: Set(Some(body.address_line1.clone())),
        ...
    };

    let result = new_addr.insert(&db).await?;
    Ok(HttpResponse::Created().json(AddressResponse::from(result)))
}
```

## 📋 Migration Checklist

When reorganizing existing code:

- [x] Move models to `internal/`
- [x] Create DTOs in `dto/`
- [x] Update `lib.rs` exports
- [x] Update handler imports
- [x] Update service imports
- [x] Run `cargo check --workspace`
- [x] Update documentation

## 🎯 Future Enhancements

- [ ] Add validation derives (e.g., `validator` crate)
- [ ] Add DTO builders for complex constructions
- [ ] Add API versioning (v1, v2 DTOs)
- [ ] Add GraphQL schema generation from DTOs
- [ ] Add OpenAPI/Swagger generation from DTOs
- [ ] Add transformation helpers (batch conversions)
- [ ] Add request/response middleware
- [ ] Add DTO testing utilities

## ✨ Summary

**Internal Models:** Database entities, managed by SeaORM
- Location: `models::internal::*`
- Use in: Data layer, queries, migrations

**DTOs:** API request/response types
- Location: `models::dto::*`
- Use in: HTTP handlers, external APIs

**Clean separation = Maintainable codebase** 🎉
