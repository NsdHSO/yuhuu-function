# User Property Consolidation Plan
**Church Management System (yuhuu-function) → Auth Server Integration**

---

## Executive Summary

Consolidate duplicate user authentication properties by using the auth server (https://github.com/NsdHSO/auth) as the **single source of truth** for authentication. Remove duplicate fields from `yuhuu-function` and establish a clean separation of concerns:

- **Auth Server**: Handles authentication, identity, and account management
- **Church System**: Handles church-specific data (membership, attendance, giving, ministry)

---

## Current State Analysis

### Auth Server Properties (Source of Truth)
Based on standard auth server implementation with JWT + email verification:

**Expected Fields:**
- `id` (UUID or similar primary key)
- `email` (unique, for authentication)
- `username` (optional, unique)
- `password_hash` (bcrypt/argon2)
- `first_name`, `last_name` (basic identity)
- `role` (enum: Admin, User, Moderator, etc.)
- `status` (enum: Active, Inactive, Suspended, PendingVerification)
- `email_verified` (boolean)
- `email_verified_at` (timestamp)
- `last_login` (timestamp)
- `created_at`, `updated_at` (timestamps)
- `refresh_token` / `session_data` (for JWT management)

### yuhuu-function Current User Model

**File**: `crates/models/src/user.rs`

**Current Fields:**
```rust
pub struct Model {
    pub id: i64,                              // Local DB ID
    pub uuid: Uuid,                           // ❌ DUPLICATE (auth has ID)
    pub first_name: String,                   // ❌ DUPLICATE
    pub last_name: String,                    // ❌ DUPLICATE
    pub email: String,                        // ❌ DUPLICATE
    pub password_hash: String,                // ❌ DUPLICATE (security risk!)
    pub is_active: bool,                      // ❌ DUPLICATE (auth has status)
    pub is_email_verified: bool,              // ❌ DUPLICATE
    pub email_verified_at: Option<DateTime>,  // ❌ DUPLICATE
    pub created_at: DateTime,                 // ⚠️  DUPLICATE (but OK for local tracking)
    pub updated_at: DateTime,                 // ⚠️  DUPLICATE (but OK for local tracking)
    pub last_login: Option<DateTime>,         // ❌ DUPLICATE
}
```

**Church-Specific Tables (NO DUPLICATES - Keep These):**
1. ✅ `user_profiles` - Contact info, demographics, emergency contacts
2. ✅ `user_addresses` - Physical addresses
3. ✅ `user_memberships` - Church membership, baptism, spiritual data
4. ✅ `user_roles` - Church roles (Pastor, Elder, Deacon, etc.)
5. ✅ `user_ministries` - Ministry participation
6. ✅ `attendances` - Attendance tracking
7. ✅ `givings` - Financial contributions

---

## Identified Problems

### 1. **Security Risk**
Storing `password_hash` locally duplicates sensitive authentication data. If compromised, attackers have two attack surfaces.

### 2. **Data Inconsistency**
User updates their email/name in auth server → Church system has stale data.

### 3. **Maintenance Burden**
Must sync user updates across two systems manually.

### 4. **Violation of Single Source of Truth**
Auth server should be the ONLY place storing authentication credentials.

### 5. **Unclear Responsibility**
Where should password resets happen? Email verification? Account lockouts?

---

## Recommended Architecture

### Approach: **HTTP REST API Integration**

**Why REST (not shared database)?**
- ✅ Clean service boundaries
- ✅ Independent deployment and scaling
- ✅ Easier to test and mock
- ✅ No database coupling
- ✅ Auth server can be language-agnostic

### User Reference Strategy

**Transform the `users` table in church database:**

```rust
// NEW: Lightweight user reference table
pub struct Model {
    pub id: i64,                    // Internal church DB ID (auto-increment)
    pub auth_user_id: String,       // Foreign key to auth server user (UUID or int)
    pub created_at: DateTime,       // When user was linked to church system
    pub updated_at: DateTime,       // Last sync/update
}
```

**This table acts as:**
- A bridge between auth server users and church data
- Maintains local `i64` IDs for existing church table relations
- References auth users via `auth_user_id` (UUID/String from auth server)

---

## Data Flow & Responsibility Matrix

| Data Type | Owner | Storage Location | Access Method |
|-----------|-------|------------------|---------------|
| Email | Auth Server | Auth DB | REST API |
| Password | Auth Server | Auth DB | REST API (never exposed) |
| First Name / Last Name | Auth Server | Auth DB | REST API |
| Email Verified | Auth Server | Auth DB | REST API |
| Account Status (Active/Inactive) | Auth Server | Auth DB | REST API |
| Last Login | Auth Server | Auth DB | REST API |
| Auth Role (Admin/User) | Auth Server | Auth DB | REST API |
| **Church ID (Bridge)** | Church System | Church DB | Direct query |
| Phone, DOB, Gender | Church System | Church DB (user_profiles) | Direct query |
| Membership Status | Church System | Church DB (user_memberships) | Direct query |
| Baptism Info | Church System | Church DB (user_memberships) | Direct query |
| Church Role (Pastor/Elder) | Church System | Church DB (user_roles) | Direct query |
| Ministry Participation | Church System | Church DB (user_ministries) | Direct query |
| Address | Church System | Church DB (user_addresses) | Direct query |
| Attendance | Church System | Church DB (attendances) | Direct query |
| Giving | Church System | Church DB (givings) | Direct query |

---

## Implementation Plan

### Phase 1: Discovery & Analysis

**1.1 Verify Auth Server API**
- [ ] Document all available auth server REST endpoints
- [ ] Identify endpoint for: get user by ID, get user by email, verify JWT
- [ ] Check if auth server has webhook support for user updates
- [ ] Determine authentication method for server-to-server calls (API key? Service account?)

**1.2 Audit Current Usage**
- [ ] Search codebase for all references to `user.email`, `user.password_hash`, `user.first_name`, etc.
- [ ] Identify where authentication happens (login, registration, password reset)
- [ ] List all queries that join on user table fields that will be removed

**1.3 Data Inventory**
- [ ] Count existing users in church database
- [ ] Determine if there's existing data in `password_hash`, `email`, etc.
- [ ] **CRITICAL DECISION**: Do we have production data that needs migration?

---

### Phase 2: Architecture Design

**2.1 Create Auth Client Service**

**File**: `crates/logic/src/auth_client.rs`

```rust
pub struct AuthClient {
    base_url: String,
    api_key: Option<String>,
    http_client: reqwest::Client,
}

impl AuthClient {
    // Fetch user details from auth server
    pub async fn get_user(&self, auth_user_id: &str) -> Result<AuthUser>;

    // Fetch user by email
    pub async fn get_user_by_email(&self, email: &str) -> Result<AuthUser>;

    // Verify JWT token (for request authentication)
    pub async fn verify_token(&self, token: &str) -> Result<TokenClaims>;

    // Check if user exists (for linking)
    pub async fn user_exists(&self, auth_user_id: &str) -> Result<bool>;
}

// Response DTO from auth server
pub struct AuthUser {
    pub id: String,                         // Auth server's user ID
    pub email: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: String,
    pub status: String,
    pub email_verified: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
```

**2.2 Create User Service Facade**

**File**: `crates/logic/src/user_service.rs`

```rust
pub struct UserService {
    db: DatabaseConnection,
    auth_client: AuthClient,
}

impl UserService {
    // Get complete user (auth data + church data)
    pub async fn get_complete_user(&self, church_user_id: i64) -> Result<CompleteUser>;

    // Link an auth user to church system
    pub async fn link_auth_user(&self, auth_user_id: &str) -> Result<ChurchUser>;

    // Lookup church user by auth ID
    pub async fn find_by_auth_id(&self, auth_user_id: &str) -> Result<Option<ChurchUser>>;
}

// Composite type combining both sources
pub struct CompleteUser {
    pub auth_user: AuthUser,           // From auth server API
    pub church_user: ChurchUserModel,  // From local DB
    pub profile: Option<UserProfileModel>,
    pub membership: Option<UserMembershipModel>,
    pub address: Option<UserAddressModel>,
}
```

**2.3 Update Database Schema**

**Migration**: `migration/src/mYYYYMMDD_HHMMSS_consolidate_user_table.rs`

```rust
// Remove columns from users table
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Drop columns
    manager.alter_table(
        Table::alter()
            .table(Users::Table)
            .drop_column(Users::Email)
            .drop_column(Users::PasswordHash)
            .drop_column(Users::FirstName)
            .drop_column(Users::LastName)
            .drop_column(Users::IsActive)
            .drop_column(Users::IsEmailVerified)
            .drop_column(Users::EmailVerifiedAt)
            .drop_column(Users::LastLogin)
            .to_owned()
    ).await?;

    // Add auth_user_id column
    manager.alter_table(
        Table::alter()
            .table(Users::Table)
            .add_column(
                ColumnDef::new(Users::AuthUserId)
                    .string()
                    .not_null()
                    .unique_key()
            )
            .to_owned()
    ).await?;

    // Add index for fast lookups
    manager.create_index(
        Index::create()
            .name("idx_users_auth_user_id")
            .table(Users::Table)
            .col(Users::AuthUserId)
            .to_owned()
    ).await?;

    Ok(())
}
```

**2.4 Update User Model**

**File**: `crates/models/src/user.rs`

```rust
/// Bridge table linking auth server users to church database
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,                    // Local church DB ID

    pub auth_user_id: String,       // References auth server user (UUID/String)

    pub created_at: DateTime,       // When linked to church system
    pub updated_at: DateTime,       // Last update
}
```

---

### Phase 3: Application Layer Updates

**3.1 Authentication Flow**

**Current** (Bad - Duplicate):
```
User Login → Check church DB → Verify password → Issue JWT
```

**New** (Good - Delegated):
```
User Login → Forward to Auth Server → Receive JWT → Store in session
User Request → Extract JWT → Verify with Auth Server → Get user ID → Query church data
```

**3.2 Registration Flow**

**Option A: Auth Server First (Recommended)**
```
1. User registers → Auth Server
2. Auth Server sends verification email
3. User verifies email
4. Church admin "links" user to church system by email/UUID
5. Church system creates record: auth_user_id = <auth_user_uuid>
6. Church admin adds church-specific data (profile, membership, etc.)
```

**Option B: Church System Proxy**
```
1. User registers via church system
2. Church system calls Auth Server API to create user
3. On success, church system creates local link record
4. Church system adds church-specific data
```

**Decision Required**: Which registration flow fits your workflow better?

**3.3 User Data Access Patterns**

**Pattern 1: Always Fetch Fresh (Real-time)**
```rust
async fn get_user_details(church_id: i64) -> Result<UserDetails> {
    // 1. Get church user record
    let church_user = ChurchUser::find_by_id(church_id).one(&db).await?;

    // 2. Fetch from auth server
    let auth_user = auth_client.get_user(&church_user.auth_user_id).await?;

    // 3. Combine data
    Ok(UserDetails {
        email: auth_user.email,
        name: format!("{} {}", auth_user.first_name, auth_user.last_name),
        phone: church_user.profile?.phone,
        // ...
    })
}
```

**Pattern 2: Cache with TTL (Performance)**
```rust
// Use Redis/in-memory cache for auth user data
// TTL: 5-15 minutes
// Invalidate on user update webhook

async fn get_user_details_cached(church_id: i64) -> Result<UserDetails> {
    let cache_key = format!("auth_user:{}", church_user.auth_user_id);

    // Try cache first
    if let Some(cached) = cache.get(&cache_key).await? {
        return Ok(cached);
    }

    // Fetch from auth server
    let auth_user = auth_client.get_user(&church_user.auth_user_id).await?;

    // Cache for 10 minutes
    cache.set(&cache_key, &auth_user, Duration::minutes(10)).await?;

    Ok(auth_user)
}
```

**Decision Required**: Real-time or cached? Trade-off between consistency vs. performance.

---

### Phase 4: Migration Strategy

**CRITICAL: Check if you have production data**

#### Scenario A: No Production Data (Clean Start)

✅ **Simple Path**
1. Update schema (drop columns, add `auth_user_id`)
2. Update code to use auth server
3. Deploy
4. Users register fresh via auth server

#### Scenario B: Production Data Exists

**Option B1: Manual Migration (Small Dataset < 100 users)**
1. Export existing users: `SELECT id, uuid, email, first_name, last_name FROM users`
2. For each user:
   - Create account in auth server via API
   - Get auth_user_id from response
   - Update church DB: `UPDATE users SET auth_user_id = ? WHERE id = ?`
3. Run schema migration to drop old columns
4. Deploy updated code

**Option B2: Automated Migration (Large Dataset)**
1. Write migration script:
   ```rust
   async fn migrate_users_to_auth_server() {
       let users = User::find().all(&db).await?;

       for user in users {
           // Create user in auth server
           let auth_user = auth_client.create_user(CreateUserRequest {
               email: user.email,
               password_hash: user.password_hash, // IF auth server supports it
               first_name: user.first_name,
               last_name: user.last_name,
               // ... other fields
           }).await?;

           // Update local record
           let mut active: UserActiveModel = user.into();
           active.auth_user_id = Set(auth_user.id);
           active.update(&db).await?;
       }
   }
   ```
2. Run migration in maintenance window
3. Verify data integrity
4. Drop old columns
5. Deploy updated code

**Option B3: Gradual Migration (Safest)**
1. Add `auth_user_id` column (nullable)
2. Deploy code that writes to BOTH systems temporarily
3. Background job migrates existing users
4. Once all users migrated, make `auth_user_id` NOT NULL
5. Drop old columns
6. Remove dual-write code

**Decision Required**: Which migration strategy? Depends on:
- How many users?
- Acceptable downtime?
- Can users re-register?

---

### Phase 5: API/Endpoint Updates

**5.1 REST API Changes**

**Current Endpoint** (assuming you have these):
```
POST /api/auth/register
POST /api/auth/login
POST /api/users/{id}
GET  /api/users/{id}
```

**New Approach**:
```
POST /api/auth/register     → Proxy to Auth Server
POST /api/auth/login        → Proxy to Auth Server
POST /api/auth/logout       → Proxy to Auth Server
POST /api/users/{id}/link   → Link existing auth user to church (admin only)
GET  /api/users/{id}        → Fetch auth data + church data combined
PUT  /api/users/{id}/profile → Update church-specific data only
```

**5.2 Authentication Middleware**

```rust
// Extract user from JWT and load church context
async fn auth_middleware(req: ServiceRequest) -> Result<ServiceRequest> {
    // 1. Extract JWT from Authorization header
    let token = extract_bearer_token(&req)?;

    // 2. Verify token with auth server
    let claims = auth_client.verify_token(&token).await?;

    // 3. Find church user by auth_user_id
    let church_user = User::find()
        .filter(user::Column::AuthUserId.eq(&claims.sub))
        .one(&db)
        .await?;

    // 4. Attach to request context
    req.extensions_mut().insert(AuthenticatedUser {
        auth_user_id: claims.sub,
        church_user_id: church_user.map(|u| u.id),
        role: claims.role,
    });

    Ok(req)
}
```

---

### Phase 6: Testing & Validation

**6.1 Integration Tests**
- [ ] Test user registration flow (auth server → church link)
- [ ] Test user login flow (JWT verification)
- [ ] Test user data retrieval (combined auth + church data)
- [ ] Test church-specific updates (profile, membership)
- [ ] Test error cases (auth server down, user not found, etc.)

**6.2 Performance Tests**
- [ ] Measure latency of auth server API calls
- [ ] Test with caching enabled/disabled
- [ ] Verify database query performance with new schema
- [ ] Load test: 100 concurrent requests

**6.3 Security Tests**
- [ ] Verify password hashes removed from church DB
- [ ] Test JWT expiration handling
- [ ] Test unauthorized access attempts
- [ ] Verify church admin can't access auth server directly

---

## Configuration Requirements

### Environment Variables

```bash
# Auth Server Configuration
AUTH_SERVER_URL=https://auth.yourchurch.com
AUTH_SERVER_API_KEY=<service_account_key>
AUTH_SERVER_TIMEOUT_MS=5000

# Cache Configuration (if using caching)
REDIS_URL=redis://localhost:6379
AUTH_CACHE_TTL_SECONDS=600

# Feature Flags
ENABLE_AUTH_CACHE=true
ENABLE_AUTH_WEBHOOKS=false
```

### Dependencies to Add

**Cargo.toml**:
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }  # For HTTP client
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
# Optional: for caching
redis = { version = "0.23", features = ["tokio-comp"] }
```

---

## Rollback Plan

**If something goes wrong after deployment:**

### Immediate Rollback (< 1 hour)
1. Revert code deployment to previous version
2. If schema was changed, run down migration
3. Re-enable old authentication endpoints

### Partial Rollback (1-24 hours)
1. Keep new schema but add fallback logic:
   ```rust
   // Try auth server first, fallback to local cache
   let user = match auth_client.get_user(id).await {
       Ok(user) => user,
       Err(_) => get_from_local_backup(id).await?
   };
   ```
2. Investigate auth server issues
3. Fix and redeploy

### Data Recovery
- Keep database backups before migration
- Retention: 30 days minimum
- Test restore procedure before migration

---

## Success Metrics

**After implementation, track:**

| Metric | Target | Current |
|--------|--------|---------|
| Auth server API response time | < 200ms | N/A |
| Failed auth server calls | < 0.1% | N/A |
| Duplicate user data | 0% | 100% |
| Security audit findings | 0 critical | ? |
| User complaints about auth | 0 | N/A |

---

## Decision Points Summary

**You need to decide:**

1. **[ ] Registration Flow**: Auth-first or Church-proxy?
2. **[ ] Data Access Pattern**: Real-time fetch or cached?
3. **[ ] Migration Strategy**: Manual, automated, or gradual?
4. **[ ] Production Data**: Do you have users that need migration?
5. **[ ] Cache Strategy**: Redis, in-memory, or no cache?
6. **[ ] Webhook Support**: Does auth server support webhooks for user updates?
7. **[ ] Auth Server API**: What endpoints are available? Need documentation.
8. **[ ] Service Account**: How does church system authenticate to auth server?

---

## Risks & Mitigation

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Auth server downtime | High | Medium | Implement circuit breaker, cache, fallback |
| Data migration failure | Critical | Low | Test thoroughly, keep backups, rollback plan |
| Performance degradation | Medium | Medium | Add caching, monitor latency, optimize queries |
| User confusion during transition | Medium | High | Clear communication, training, support docs |
| Lost user accounts | Critical | Low | Careful migration script, validation, dry runs |
| Auth server API changes | Medium | Medium | Version API calls, monitor deprecations |

---

## Timeline Estimate

**Assuming 1 full-time developer:**

- **Phase 1** (Discovery): 1-2 days
- **Phase 2** (Architecture Design): 2-3 days
- **Phase 3** (Code Changes): 5-7 days
- **Phase 4** (Migration Script): 2-3 days (if data exists)
- **Phase 5** (API Updates): 2-3 days
- **Phase 6** (Testing): 3-5 days
- **Deployment & Monitoring**: 1-2 days

**Total**: 3-4 weeks (with data migration)
**Total**: 2-3 weeks (no data migration)

---

## Next Steps

1. **[ ] Review this plan with team**
2. **[ ] Make decisions on all decision points above**
3. **[ ] Get auth server API documentation**
4. **[ ] Check if production data exists**
5. **[ ] Set timeline and assign resources**
6. **[ ] Create detailed implementation tickets**
7. **[ ] Schedule deployment window**

---

## Appendix: Alternative Architectures Considered

### Alternative 1: Shared Database (Not Recommended)
Both services connect to same PostgreSQL instance.

**Pros**: No API calls, faster queries
**Cons**: Tight coupling, deployment complexity, schema conflicts

### Alternative 2: Event-Driven Sync (Complex)
Auth server publishes events, church system listens and syncs.

**Pros**: Eventually consistent, no direct API calls
**Cons**: Complexity, eventual consistency issues, event ordering

### Alternative 3: Keep Duplicate Data (Status Quo - Not Recommended)
Continue storing auth data locally.

**Pros**: No work required
**Cons**: Security risk, data inconsistency, maintenance burden

**Recommendation**: HTTP REST API (as described in this plan)

---

**Document Version**: 1.0
**Last Updated**: 2026-02-05
**Author**: Engineering Team
**Status**: Draft - Awaiting Approval
