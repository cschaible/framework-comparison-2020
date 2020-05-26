#!/bin/bash

# Get the number of cores
CPU_CORES=$(grep -c ^processor /proc/cpuinfo)

# If there are more than 3 cores, spare 2 cores for the app, db, etc.
if [[ $CPU_CORES > 3 ]]; then
  CPU_CORES=$((CPU_CORES-2))
fi

# Start workers
for CPU in $(seq 1 $CPU_CORES); do
  locust --host http://127.0.0.1:8080 --worker &
done

# Start the master
locust --host http://127.0.0.1:8080 -u 50 -r 5 --master

# Trap to stop all workers
trap "pkill -P $$" EXIT
