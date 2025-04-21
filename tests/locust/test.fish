#!/usr/bin/env fish

set LOCUST_FILE "main.py"
set OUTPUT_DIR "logs/test"
set HOST "http://localhost:8080"
set USERS 500
set RATE 10
set DURATION "1m"

mkdir -p $OUTPUT_DIR

function run_locust_test
    uv run locust -f $LOCUST_FILE \
        --headless \
        --host=$HOST \
        --users=$USERS \
        --spawn-rate=$RATE \
        --run-time=$DURATION \
        --csv="$OUTPUT_DIR/$TEST_NAME" \
        > "$OUTPUT_DIR/$TEST_NAME.log" 2>&1
end

if not command -v uv > /dev/null
    echo "uv not found. Please install it (e.g., 'pip install uv')"
    exit 1
end

if not test -f $LOCUST_FILE
    echo "Locust file not found at $LOCUST_FILE"
    exit 1
end

echo "Cleaning up old Locust processes..."
pkill -f "locust -f $LOCUST_FILE"; or echo "No old Locust processes found"
sleep 1

run_locust_test

