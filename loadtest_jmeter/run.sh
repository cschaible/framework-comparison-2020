#!/bin/bash

DURATION=120
TEST_DURATION="$1"

if [[ -n "$TEST_DURATION" ]]; then
  DURATION=$TEST_DURATION
fi

printf "Run test for $DURATION seconds\n\n"

CURRENT_DATE_TIME="`date +%Y%m%d_%H%M%S`";
REPORT_DIR="REPORT_$CURRENT_DATE_TIME"
jmeter -n -t loadtest.jmx -o "$REPORT_DIR" -e -l "$REPORT_DIR/log.jtl" -Jduration=$DURATION
