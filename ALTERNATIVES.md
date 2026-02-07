# User Property Consolidation - Alternatives

## The Problem (Summary)
You have **duplicate user authentication data** in two places:
1. **Auth Server** - Already deployed at https://github.com/NsdHSO/auth
2. **Church System** - Your yuhuu-function storing same data (email, password, name, etc.)

**Duplicates**: `email`, `password_hash`, `first_name`, `last_name`, `is_active`, `is_email_verified`, `email_verified_at`, `last_login`

---

## Alternative 1: ✅ **RECOMMENDED - Full Consolidation**
**Use auth server as single source of truth**

### What Changes
- **Remove** all auth-related fields from church database
- **Add** `auth_user_id` to link to auth server
- **Create** REST client to fetch user data from auth server when needed
- **Keep** all church-specific data (profiles, memberships, attendance, giving)

### Architecture
```
Auth Server (owns authentication)
    ↕ REST API calls
Church System (owns church data)
```

### Pros
✅ **Single source of truth** - no data inconsistency
✅ **Better security** - no password hashes in church DB
✅ **Clear responsibility** - auth server handles auth, church handles church data
✅ **Easier maintenance** - update user email once, everywhere sees it
✅ **Scalable** - can add more services using same auth server

### Cons
❌ **Network dependency** - if auth server is down, can't get user names/emails
❌ **Latency** - API call adds ~50-200ms per request
❌ **More complexity** - need to maintain REST client code
❌ **Migration needed** - if you have existing users

### When to Choose This
- You want proper microservices architecture
- You plan to add more services in the future
- Security is important (no password duplication)
- You can tolerate slight latency for user data

### Effort
- **3-4 weeks** with existing data migration
- **2-3 weeks** for clean start

---

## Alternative 2: ⚠️ **Status Quo - Keep Everything As Is**
**Do nothing, keep duplicates**

### What Changes
- Nothing changes
- Both systems keep storing same data

### Pros
✅ **Zero effort** - no development needed
✅ **No risk** - nothing breaks
✅ **Fast queries** - everything in local database
✅ **No network dependency** - works offline

### Cons
❌ **Data inconsistency** - user changes email in auth server, church system doesn't know
❌ **Security risk** - two places storing password hashes
❌ **Maintenance burden** - must update user data in two places
❌ **Confusion** - which system is source of truth?
❌ **Sync problems** - users can have different emails in each system

### When to Choose This
- You have urgent deadlines and can't afford the time
- Auth server is just for testing/not production
- You plan to sunset one of the systems soon
- Team is too small to handle integration

### Effort
- **0 days** - no work

---

## Alternative 3: 🔄 **Hybrid - Selective Sync**
**Keep local cache of critical fields only, sync regularly**

### What Changes
- **Keep** `email`, `first_name`, `last_name` locally (cached)
- **Remove** `password_hash`, `is_email_verified` (security-sensitive)
- **Add** background job to sync user data every N minutes
- **Add** `auth_user_id` and `last_synced_at` fields

### Architecture
```
Auth Server (primary)
    ↕ Background sync job (every 5-10 min)
Church System (cached copy)
```

### Implementation
```rust
// Background job runs every 10 minutes
async fn sync_users_from_auth_server() {
    let church_users = User::find().all(&db).await?;

    for church_user in church_users {
        // Fetch latest from auth server
        let auth_user = auth_client.get_user(&church_user.auth_user_id).await?;

        // Update local cache
        church_user.email = auth_user.email;
        church_user.first_name = auth_user.first_name;
        church_user.last_name = auth_user.last_name;
        church_user.last_synced_at = now();
        church_user.save(&db).await?;
    }
}
```

### Pros
✅ **Fast queries** - data is local, no API calls during requests
✅ **Eventual consistency** - syncs regularly
✅ **Better security** - remove password hashes
✅ **Failover** - if auth server down, cached data still works
✅ **Lower latency** - no real-time API calls

### Cons
❌ **Stale data** - up to 10 minutes old
❌ **Still some duplication** - cached fields are duplicated
❌ **Sync complexity** - need background job, error handling
❌ **Storage cost** - still storing some duplicates
⚠️ **Confusion** - is cached data always accurate?

### When to Choose This
- Performance is critical (can't afford API latency)
- You need offline capability
- You can tolerate slightly stale data (5-10 min old)
- You want to remove password hashes but keep names/emails local

### Effort
- **1-2 weeks** for sync job + remove password fields

---

## Alternative 4: 🎭 **Read-Through Cache Pattern**
**Fetch from auth server on-demand, cache results**

### What Changes
- **Remove** all auth fields from database
- **Add** Redis/in-memory cache layer
- **Add** `auth_user_id` field
- **Fetch** from auth server on first access, cache for 15 minutes

### Architecture
```
Request → Check Cache → Cache Miss? → Auth Server API → Cache Result
                    → Cache Hit? → Return Cached
```

### Implementation
```rust
async fn get_user_email(church_user_id: i64) -> Result<String> {
    let church_user = User::find_by_id(church_user_id).one(&db).await?;
    let cache_key = format!("auth_user:{}", church_user.auth_user_id);

    // Try cache first
    if let Some(cached) = cache.get(&cache_key).await? {
        return Ok(cached.email);
    }

    // Cache miss - fetch from auth server
    let auth_user = auth_client.get_user(&church_user.auth_user_id).await?;

    // Cache for 15 minutes
    cache.set(&cache_key, &auth_user, Duration::minutes(15)).await?;

    Ok(auth_user.email)
}
```

### Pros
✅ **No duplicate storage** - data only in auth server
✅ **Good performance** - cached after first request
✅ **Fresh data** - short cache TTL keeps data recent
✅ **Better security** - no password hashes stored
✅ **Simple** - no background jobs needed

### Cons
❌ **First request slow** - cache miss = API call
❌ **Cache infrastructure** - need Redis or similar
❌ **Cache invalidation** - hard to know when to clear cache
❌ **Memory cost** - storing cached data in Redis
❌ **Network dependency** - cache miss = need auth server

### When to Choose This
- You have Redis already
- You want best of both worlds (speed + freshness)
- Your traffic patterns allow warming cache
- You can handle occasional slow first requests

### Effort
- **2-3 weeks** for cache layer + integration

---

## Alternative 5: 💾 **Minimal Bridge - ID Only**
**Only store auth_user_id, fetch everything else always**

### What Changes
- **Remove** ALL user fields except `id` and `auth_user_id`
- **Fetch** user data from auth server on EVERY request
- **Keep** all church-specific tables unchanged

### Users Table
```rust
pub struct Model {
    pub id: i64,              // Church DB ID
    pub auth_user_id: String, // Link to auth server
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
// That's it! No name, email, nothing.
```

### Pros
✅ **Simplest** - minimal schema, minimal storage
✅ **Always fresh** - no stale data possible
✅ **True single source of truth** - zero duplication
✅ **Best security** - nothing sensitive stored locally

### Cons
❌ **Slow** - API call on every request
❌ **High dependency** - auth server MUST be up
❌ **Network cost** - lots of API calls
❌ **No offline mode** - can't work without auth server

### When to Choose This
- Auth server is super reliable (99.99% uptime)
- You have very few users (< 100)
- Requests are infrequent
- You value simplicity over performance

### Effort
- **1-2 weeks** - minimal code changes

---

## Alternative 6: 🔀 **Two-Phase Migration**
**Gradual transition to avoid big-bang deployment**

### Phase 1 (Safe, No Breaking Changes)
- **Add** `auth_user_id` field (nullable)
- **Keep** existing fields
- **Dual-write** - update both local DB and auth server
- **Link** existing users to auth server in background

### Phase 2 (After All Users Migrated)
- **Make** `auth_user_id` required
- **Remove** duplicate fields
- **Stop** dual-writing
- **Use** auth server as source of truth

### Pros
✅ **Low risk** - gradual rollout
✅ **Can rollback** - old fields still there during phase 1
✅ **Test in production** - verify before full commit
✅ **No downtime** - seamless transition

### Cons
❌ **Takes longer** - 2 separate deployments
❌ **More code** - temporary dual-write logic
❌ **Complexity** - managing migration state
❌ **Temporary inconsistency** - data can drift during phase 1

### When to Choose This
- You have production users
- You can't afford downtime
- Risk tolerance is low
- You want to test before full commitment

### Effort
- **Phase 1**: 2 weeks
- **Wait period**: 1-2 weeks (monitoring)
- **Phase 2**: 1 week
- **Total**: 4-5 weeks

---

## Comparison Matrix

| Alternative | Effort | Risk | Performance | Security | Complexity |
|-------------|--------|------|-------------|----------|------------|
| 1. Full Consolidation | Medium | Medium | Medium | ⭐⭐⭐⭐⭐ | Medium |
| 2. Status Quo | None | High | ⭐⭐⭐⭐⭐ | ⭐ | Low |
| 3. Selective Sync | Medium | Low | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | High |
| 4. Cache Pattern | Medium | Low | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | Medium |
| 5. Minimal Bridge | Low | High | ⭐ | ⭐⭐⭐⭐⭐ | Low |
| 6. Two-Phase | High | Low | Medium | ⭐⭐⭐⭐⭐ | High |

---

## Decision Framework

### Choose **Alternative 1 (Full Consolidation)** if:
- ✅ You're building for long-term
- ✅ Security is important
- ✅ You plan to have multiple services
- ✅ You can tolerate slight latency

### Choose **Alternative 2 (Status Quo)** if:
- ✅ Auth server is not production-ready yet
- ✅ You have urgent deadlines (< 1 month)
- ✅ You plan to merge the systems later
- ⚠️ Accept the security/consistency risks

### Choose **Alternative 3 (Selective Sync)** if:
- ✅ Performance is critical
- ✅ You need offline capability
- ✅ You can tolerate 5-10 minute stale data
- ✅ You want to remove password hashes first

### Choose **Alternative 4 (Cache Pattern)** if:
- ✅ You already have Redis
- ✅ You want good performance + security
- ✅ Most data access is reads (not writes)
- ✅ Cache infrastructure is not a problem

### Choose **Alternative 5 (Minimal Bridge)** if:
- ✅ You have very few users
- ✅ Auth server is super reliable
- ✅ Simplicity > performance
- ✅ You don't mind API calls on every request

### Choose **Alternative 6 (Two-Phase)** if:
- ✅ You have production data
- ✅ Risk tolerance is very low
- ✅ You can afford longer timeline
- ✅ You want to test incrementally

---

## My Recommendation

**For your situation, I recommend: Alternative 4 (Cache Pattern)**

### Why?
1. **Best balance** - Good performance, good security
2. **Proven pattern** - Industry standard approach
3. **Flexible** - Can adjust cache TTL based on needs
4. **Removes passwords** - Better security immediately
5. **Reasonable effort** - 2-3 weeks is manageable

### Implementation Priority
1. **Week 1**: Build auth client + cache layer
2. **Week 2**: Migrate schema, remove password fields
3. **Week 3**: Test, deploy, monitor

### Fallback Plan
If cache doesn't work well, can:
- **Increase TTL** → closer to Alternative 3 (Selective Sync)
- **Remove cache** → Alternative 1 (Full Consolidation)
- **Add cache warming** → proactively populate cache

---

## Quick Start (If Choosing Alternative 4)

```bash
# 1. Add Redis
docker run -d -p 6379:6379 redis:alpine

# 2. Add dependencies to Cargo.toml
redis = { version = "0.23", features = ["tokio-comp", "connection-manager"] }
serde = { version = "1.0", features = ["derive"] }

# 3. Create auth client (crates/logic/src/auth_client.rs)
# 4. Add caching layer (crates/logic/src/cache.rs)
# 5. Update user service to use cache
# 6. Create migration to remove password fields
# 7. Deploy and monitor
```

---

## Questions to Help You Decide

1. **Do you have production users now?**
   - Yes → Alternative 6 (Two-Phase)
   - No → Alternative 1 or 4

2. **How important is performance?**
   - Critical → Alternative 3 or 4
   - Important → Alternative 4
   - Not critical → Alternative 1

3. **Is auth server production-ready?**
   - Yes → Alternative 1, 3, 4, or 6
   - No → Alternative 2 (for now)

4. **Do you have Redis/cache infrastructure?**
   - Yes → Alternative 4
   - No → Alternative 1 or 3

5. **How much time do you have?**
   - < 1 week → Alternative 2
   - 1-2 weeks → Alternative 5
   - 2-3 weeks → Alternative 4
   - 3-4 weeks → Alternative 1
   - 4-5 weeks → Alternative 6

---

**What do you think? Which alternative fits your situation best?**
