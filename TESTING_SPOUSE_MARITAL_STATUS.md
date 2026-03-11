# Spouse Marital Status Auto-Update Testing

## Overview

This document describes the testing strategy for the automatic marital status update feature when users create, update, or delete spouse relationships.

## Feature Behavior

### Auto-Update Rules

1. **Create Spouse Relationship**
   - Always sets `marital_status = "Married"`
   - If user has no gender AND spouse has gender → infer opposite gender
   - If user already has gender → keep it unchanged

2. **Delete Spouse Relationship**
   - If user has NO other spouse relationships → set `marital_status = "Single"`
   - If user has other spouse relationships → keep `marital_status = "Married"`

3. **Update TO Spouse** (from other relationship type)
   - Same as creating spouse relationship
   - Sets `marital_status = "Married"`
   - Infers gender if needed

4. **Update FROM Spouse** (to other relationship type)
   - Same as deleting spouse relationship
   - Sets `marital_status = "Single"` (if no other spouses)

## Test Files

### 1. Unit Tests
**Location:** `crates/functions/src/features/family_relationships/service.rs`

**Run with:**
```bash
cargo test --package functions --lib family_relationships::service::tests
```

**Tests:**
- `test_spouse_relationship_type_lowercase_normalization` - Validates case-insensitive comparison
- `test_create_request_validation_requires_related_info` - Validates request structure
- `test_update_request_can_change_relationship_type` - Validates update request
- `test_gender_inference_logic` - Validates opposite gender logic

### 2. Integration Tests
**Location:** `crates/functions/tests/spouse_marital_status_integration.rs`

**Run with:**
```bash
cargo test --test spouse_marital_status_integration
```

**Status:** Placeholder (requires test database setup)

### 3. Manual Integration Test Script
**Location:** `test-spouse-marital-status.sh`

**Run with:**
```bash
chmod +x test-spouse-marital-status.sh
./test-spouse-marital-status.sh
```

**Prerequisites:**
1. Server running on `localhost:2003`
2. Valid JWT token (set in script)
3. `jq` installed for JSON parsing

## Test Scenarios

### Scenario 1: New User Creates Spouse
```
1. Bootstrap user → marital_status: null
2. Update profile → gender: "Male", marital_status: "Single"
3. Create spouse relationship
4. Verify → marital_status: "Married"
```

**Expected Result:** ✅ Marital status auto-updated to "Married"

### Scenario 2: User Deletes Spouse
```
1. User has spouse → marital_status: "Married"
2. Delete spouse relationship
3. Verify → marital_status: "Single"
```

**Expected Result:** ✅ Marital status auto-updated to "Single"

### Scenario 3: Gender Inference
```
1. User A → gender: null
2. User B → gender: "Female"
3. User A creates spouse relationship to User B
4. Verify → User A gender: "Male"
```

**Expected Result:** ✅ Gender inferred from spouse

### Scenario 4: Gender Not Changed
```
1. User A → gender: "Male"
2. User B → gender: "Female"
3. User A creates spouse relationship to User B
4. Verify → User A gender: "Male" (unchanged)
```

**Expected Result:** ✅ Gender preserved

### Scenario 5: Update Relationship Type
```
1. User has "sibling" relationship
2. Update relationship_type to "spouse"
3. Verify → marital_status: "Married"
```

**Expected Result:** ✅ Marital status auto-updated

### Scenario 6: Multiple Spouses (Edge Case)
```
1. User has 2 spouse relationships
2. Delete 1 spouse
3. Verify → marital_status: "Married" (still has 1 spouse)
4. Delete 2nd spouse
5. Verify → marital_status: "Single"
```

**Expected Result:** ✅ Smart marital status based on remaining spouses

## Running All Tests

### Unit Tests Only
```bash
cargo test --package functions
```

### Integration Tests (when implemented)
```bash
cargo test --test spouse_marital_status_integration
```

### Manual End-to-End Test
```bash
# 1. Start the server
cargo run

# 2. In another terminal, run the test script
./test-spouse-marital-status.sh
```

## Test Data Cleanup

After running manual tests, clean up test data:

```sql
-- Delete test relationships
DELETE FROM family_relationships WHERE user_id IN (SELECT id FROM users WHERE auth_user_id LIKE 'test-%');

-- Delete test profiles
DELETE FROM user_profiles WHERE user_id IN (SELECT id FROM users WHERE auth_user_id LIKE 'test-%');

-- Delete test users
DELETE FROM users WHERE auth_user_id LIKE 'test-%';
```

## Known Limitations

1. Integration tests require test database setup (not yet implemented)
2. Manual test script requires manual JWT token insertion
3. Gender inference only works with registered users (related_user_id), not external persons

## Future Improvements

1. Implement proper integration tests with test database
2. Add GitHub Actions CI pipeline for automated testing
3. Add property-based testing for edge cases
4. Add performance tests for bulk operations
