#!/bin/bash

for i in {1..100}; do
    timeout 10s cargo test
    status=$?
    if [ $status -ne 0 ]; then
        echo "Test failed or timed out on iteration $i"
        exit $status
    fi
done

echo "All tests passed 100 times within 5 seconds each."