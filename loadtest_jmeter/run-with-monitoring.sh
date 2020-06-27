#!/bin/bash

# Configure test duration
TEST_DURATION="$1"
DURATION=120

if [[ -n "$TEST_DURATION" ]]; then
  DURATION=$TEST_DURATION
fi
MONITORING_DURATION=$(($DURATION + 15))

# Configure test (directory) name
TEST_NAME="$2"
NAME="$CURRENT_DATE_TIME"

TEST_DISPLAY_NAME="$3"
DISPLAY_NAME="$TEST_NAME"

CURRENT_DATE_TIME="`date +%Y%m%d_%H%M%S`";

if [[ -n "$TEST_NAME" ]]; then
  NAME=$TEST_NAME
fi
if [[ -n "$TEST_DISPLAY_NAME" ]]; then
  DISPLAY_NAME=$TEST_DISPLAY_NAME
fi
REPORT_DIR="REPORT_$NAME"

printf "\nRun test for $DURATION seconds\n\n"

# Start resource monitoring
CONTAINER_PID=$(docker inspect -f '{{.State.Pid}}' footballmanager)
psrecord $CONTAINER_PID --include-children --duration $MONITORING_DURATION --interval 0.5 --plot "$REPORT_DIR/$NAME-resources-plot.png" &
MONITORING_PID=$!
trap "kill $MONITORING_PID" INT

# Run load test
jmeter -n -t loadtest.jmx -o "$REPORT_DIR" -e -l "$REPORT_DIR/log.jtl" -Jduration=$DURATION

# Generate images and html
cd "$REPORT_DIR"
IMG_WIDTH=900
IMG_HEIGHT=300
IMG_GRANULARITY=1000
IMG_AUTO_SCALE=yes
JMeterPluginsCMD.sh --generate-png "$NAME-response-time-over-time.png" --input-jtl log.jtl --plugin-type ResponseTimesOverTime --width $IMG_WIDTH --height $IMG_HEIGHT --prevent-outliers no --paint-gradient no --granulation $IMG_GRANULARITY --auto-scale $IMG_AUTO_SCALE
JMeterPluginsCMD.sh --generate-png "$NAME-response-time-percentiles.png" --input-jtl log.jtl --plugin-type ResponseTimesPercentiles --width $IMG_WIDTH --height $IMG_HEIGHT --prevent-outliers no --paint-gradient no --granulation $IMG_GRANULARITY --auto-scale $IMG_AUTO_SCALE
JMeterPluginsCMD.sh --generate-png "$NAME-hits-per-second.png" --input-jtl log.jtl --plugin-type HitsPerSecond --width $IMG_WIDTH --height $IMG_HEIGHT --prevent-outliers no --paint-gradient no --granulation $IMG_GRANULARITY --auto-scale $IMG_AUTO_SCALE
JMeterPluginsCMD.sh --generate-png "$NAME-transactions-per-second.png" --input-jtl log.jtl --plugin-type TransactionsPerSecond --width $IMG_WIDTH --height $IMG_HEIGHT --prevent-outliers no --paint-gradient no --granulation $IMG_GRANULARITY --auto-scale $IMG_AUTO_SCALE
JMeterPluginsCMD.sh --generate-png "$NAME-response-time-percentiles.png" --input-jtl log.jtl --plugin-type ResponseTimesPercentiles --width $IMG_WIDTH --height $IMG_HEIGHT --prevent-outliers no --paint-gradient no --granulation $IMG_GRANULARITY --auto-scale $IMG_AUTO_SCALE
JMeterPluginsCMD.sh --generate-png "$NAME-codes-per-second.png" --input-jtl log.jtl --plugin-type ResponseCodesPerSecond --width $IMG_WIDTH --height $IMG_HEIGHT --prevent-outliers no --paint-gradient no --granulation $IMG_GRANULARITY --auto-scale $IMG_AUTO_SCALE
JMeterPluginsCMD.sh --generate-png "$NAME-bytes-over-time.png" --input-jtl log.jtl --plugin-type BytesThroughputOverTime --width $IMG_WIDTH --height $IMG_HEIGHT --prevent-outliers no --paint-gradient no --granulation $IMG_GRANULARITY --auto-scale $IMG_AUTO_SCALE

cat >report.html <<EOL
<b>$DISPLAY_NAME</b>
<html>
<table style="width: 100%">
<tr>
<td><img src="$NAME-response-time-over-time.png"/></td>
<td><img src="$NAME-response-time-percentiles.png"/></td>
</tr>
<tr>
<td><img src="$NAME-hits-per-second.png"/></td>
<td><img src="$NAME-transactions-per-second.png"/></td>
</tr>
<tr>
<td><img src="$NAME-codes-per-second.png"/></td>
<td><img src="$NAME-bytes-over-time.png"/></td>
</tr>
<tr><td colspan="2"><img height="300px" width="500px" src="$NAME-resources-plot.png"/></td></tr>
</table>
</html>
EOL

cd -
