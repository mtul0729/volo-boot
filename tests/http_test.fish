#!/usr/bin/env fish

set BASE_URL "http://localhost:8080"

if test (count $argv) -eq 0
    set DURATION 60
else
    set DURATION $argv[1]
end

function test_get_user
    echo "Testing get_user endpoint..."
    curl -X GET "$BASE_URL/user/query-one?id=1" -H "Content-Type: application/json"
    echo -e "\n"
end

function test_get_order
    echo "Testing get_order endpoint..."
    curl -X GET "$BASE_URL/order/query-one?id=1" -H "Content-Type: application/json"
    echo -e "\n"
end

set END_TIME (math (date +%s) + $DURATION)

echo "Starting API tests for $DURATION seconds..."

while test (date +%s) -lt $END_TIME
    echo "Starting new test cycle at" (date)
    test_get_user
    test_get_order
    echo "----------------------------------------"
end

echo "Finished testing after $DURATION seconds"