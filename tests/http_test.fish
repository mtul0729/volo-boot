#!/usr/bin/env fish

set BASE_URL "http://localhost:8080"

function test_get_user
    echo "Testing get_user endpoint..."
    curl -X GET "$BASE_URL/user/query-one?id=1" -H "Content-Type: application/json"
end

function test_get_order
    echo "Testing get_order endpoint..."
    curl -X GET "$BASE_URL/order/query-one?id=1" -H "Content-Type: application/json"
end

test_get_user
test_get_order

