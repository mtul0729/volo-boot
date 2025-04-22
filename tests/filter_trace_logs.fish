#!/usr/bin/env fish

set LOG_FILE $argv[1]
if test -z "$LOG_FILE"
    set LOG_FILE "../order.log"
end

cat $LOG_FILE | grep 'trace_id=' | grep 'request:' | while read line
    set request_id (echo $line | grep -o 'request_id=[^,]*' | cut -d'=' -f2)
    set trace_id (echo $line | grep -o 'trace_id=[^,]*' | cut -d'=' -f2)
    set span_id (echo $line | grep -o 'span_id=[^,]*' | cut -d'=' -f2)
    set parent_id (echo $line | grep -o 'parent_id=[^,]*' | cut -d'=' -f2)
    set service (echo $line | grep -o 'service=[^,]*' | cut -d'=' -f2)
    set timestamp (echo $line | grep -o '^[^ ]*')

    echo "{\"timestamp\": \"$timestamp\", \"request_id\": \"$request_id\", \"trace_id\": \"$trace_id\", \"span_id\": \"$span_id\", \"parent_id\": \"$parent_id\", \"service\": \"$service\"}"
end

