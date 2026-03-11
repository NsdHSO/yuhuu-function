#!/bin/bash

# Test script for spouse marital status auto-update functionality
# This script tests the automatic marital status updates when creating/deleting spouse relationships

set -e

BASE_URL="http://localhost:2003/v1"
TOKEN="YOUR_JWT_TOKEN_HERE"

echo "🧪 Testing Spouse Marital Status Auto-Update"
echo "============================================="

# Helper function to make authenticated requests
function api_call() {
    local method=$1
    local endpoint=$2
    local data=$3

    if [ -n "$data" ]; then
        curl -s -X "$method" "$BASE_URL$endpoint" \
            -H "Authorization: Bearer $TOKEN" \
            -H "Content-Type: application/json" \
            -d "$data"
    else
        curl -s -X "$method" "$BASE_URL$endpoint" \
            -H "Authorization: Bearer $TOKEN"
    fi
}

echo ""
echo "📋 Test 1: Bootstrap user and create empty profile"
BOOTSTRAP_RESPONSE=$(api_call POST "/bootstrap" '{"create_profile_if_missing": true}')
echo "$BOOTSTRAP_RESPONSE" | jq .

USER_ID=$(echo "$BOOTSTRAP_RESPONSE" | jq -r '.message.user.id')
echo "User ID: $USER_ID"

echo ""
echo "📋 Test 2: Verify profile has null marital_status initially"
PROFILE_RESPONSE=$(api_call GET "/users/$USER_ID/profile")
MARITAL_STATUS=$(echo "$PROFILE_RESPONSE" | jq -r '.message.marital_status')
echo "Marital Status: $MARITAL_STATUS (should be null)"

echo ""
echo "📋 Test 3: Set user gender to Male and marital_status to Single"
UPDATE_PROFILE=$(api_call PUT "/users/$USER_ID/profile" '{
    "gender": "Male",
    "marital_status": "Single"
}')
echo "$UPDATE_PROFILE" | jq .

echo ""
echo "📋 Test 4: Create spouse relationship (external person)"
CREATE_SPOUSE=$(api_call POST "/profiles/me/family" '{
    "related_person_name": "Maria Smith",
    "relationship_type": "spouse"
}')
echo "$CREATE_SPOUSE" | jq .
SPOUSE_ID=$(echo "$CREATE_SPOUSE" | jq -r '.message.id')
echo "Spouse Relationship ID: $SPOUSE_ID"

echo ""
echo "📋 Test 5: Verify marital_status auto-updated to Married"
PROFILE_AFTER_SPOUSE=$(api_call GET "/users/$USER_ID/profile")
MARITAL_STATUS_AFTER=$(echo "$PROFILE_AFTER_SPOUSE" | jq -r '.message.marital_status')
echo "Marital Status: $MARITAL_STATUS_AFTER (should be 'Married')"

if [ "$MARITAL_STATUS_AFTER" == "Married" ]; then
    echo "✅ Test 5 PASSED: Marital status auto-updated to Married"
else
    echo "❌ Test 5 FAILED: Expected 'Married', got '$MARITAL_STATUS_AFTER'"
fi

echo ""
echo "📋 Test 6: Delete spouse relationship"
DELETE_RESPONSE=$(api_call DELETE "/profiles/me/family/$SPOUSE_ID")
echo "$DELETE_RESPONSE" | jq .

echo ""
echo "📋 Test 7: Verify marital_status auto-updated back to Single"
PROFILE_AFTER_DELETE=$(api_call GET "/users/$USER_ID/profile")
MARITAL_STATUS_FINAL=$(echo "$PROFILE_AFTER_DELETE" | jq -r '.message.marital_status')
echo "Marital Status: $MARITAL_STATUS_FINAL (should be 'Single')"

if [ "$MARITAL_STATUS_FINAL" == "Single" ]; then
    echo "✅ Test 7 PASSED: Marital status auto-updated to Single after delete"
else
    echo "❌ Test 7 FAILED: Expected 'Single', got '$MARITAL_STATUS_FINAL'"
fi

echo ""
echo "============================================="
echo "🎉 All tests completed!"
