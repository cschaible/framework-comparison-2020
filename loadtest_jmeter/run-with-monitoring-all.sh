#!/bin/bash

function run_load_test() {
  DURATION="$1"
  DIR_NAME="$2"
  DESCRIPTION="$3"
  
  ./run-with-monitoring.sh $DURATION "$DIR_NAME" "$DESCRIPTION"
}

function start_framework() {
  FRAMEWORK_DIR="$1"
  START_COMMAND="$2"
  BUILD_COMMAND="$3"
  PRE_BUILD_COMMAND="$4"
  BUILD="$5"
  
  # Run pre-build command if defined
  if [[ -n "$BUILD" && -n "$PRE_BUILD_COMMAND" ]]; then
    echo "$PRE_BUILD_COMMAND" 
    $PRE_BUILD_COMMAND
  fi
  
  # Build if requested
  if [[ -n "$BUILD" && "$BUILD" == "build" ]]; then
    ./$BUILD_COMMAND
  fi
  
  # Start the framework
  ./$START_COMMAND
  
  # Wait until the app is started and ready to process requests
  while [[ "$(curl -X GET --write-out %{http_code} --silent --output /dev/null http://localhost:8080/footballers)" != "200" ]]; do 
    echo "Waiting for ${FRAMEWORK_DIR} ...";
    sleep 5;
  done
}

function stop_framework() {
  ./down.sh
}

function reset_db() {
  ../docker/reset-db.sh
}

function write_report_header() {
  REPORT_FILE="$1"
  
cat << EOT >> $REPORT_FILE
<html>
<body>
<table style="width: 100%">
EOT
}

function write_report_footer() {
  REPORT_FILE="$1"
  
cat << EOT >> $REPORT_FILE
</table>
</body>
</html>
EOT
}

function write_metric_header() {
  REPORT_FILE="$1"
  NUMBER_OF_COLUMNS="$2"
  TITLE="$3"
  
cat << EOT >> $REPORT_FILE
<tr>
  <td colspan="${NUMBER_OF_COLUMNS}">
    <br/>
    <h3>${TITLE}</h3>
  </td>
</tr>
EOT
}

function write_metric_img() {
  REPORT_FILE="$1"
  FRAMEWORK="$2"
  IMG="$3"
  
cat << EOT >> $REPORT_FILE
  <td>
    <b>${FRAMEWORK}</b>
    <br/>
    <img width="100%" src="img/${IMG}"/>
    <br/>
  </td>
EOT
}

function write_report_new_row() {
  REPORT_FILE="$1"
  
cat << EOT >> $REPORT_FILE
<tr>
EOT
}

function write_report_end_row() {
  REPORT_FILE="$1"
  
cat << EOT >> $REPORT_FILE
</tr>
EOT
}

function write_report_empty_column() {
  REPORT_FILE="$1"
  
cat << EOT >> $REPORT_FILE
  <td></td>
EOT
}

function write_report_section() {
  local -n FRAMEWORKS_REF=$1
  REPORT_FILE="$2"
  CURRENT_DATE_TIME="$3"
  IMG_NAME="$4"
  TITLE="$5"
  NUMBER_OF_COLUMNS="$6"

  # Add row with metric title to the report
  write_metric_header "$REPORT_FILE" "${NUMBER_OF_COLUMNS}" "${TITLE}"
  
  # Sort frameworks alphabetically
  declare -a FRAMEWORK_NAMES
  for NAME in "${!FRAMEWORKS_REF[@]}"; do FRAMEWORK_NAMES+=($NAME); done
  readarray -t SORTED_FRAMEWORKS < <(printf '%s\0' "${FRAMEWORK_NAMES[@]}" | sort -z | xargs -0n1)

  i=0
  for FRAMEWORK in "${SORTED_FRAMEWORKS[@]}"; do

    # Start new line if necessary
    if [[ $(($i % $NUMBER_OF_COLUMNS)) == 0 ]]; then
      write_report_new_row "$REPORT_FILE"
    fi

    # Copy image from framework report to the report directory  
    IMG="${CURRENT_DATE_TIME}_${FRAMEWORK}-${IMG_NAME}"
    cp "${REPORT_DIR}_${FRAMEWORK}/${IMG}" "${REPORT_DIR}/img/${IMG}"  

    # Add cell with framework name and picture to the report
    write_metric_img "$REPORT_FILE" "${FRAMEWORKS_REF[$FRAMEWORK]}" "${IMG}"
    
    # Finish line if necessary
    if [[ $(($i % $NUMBER_OF_COLUMNS)) == $(($NUMBER_OF_COLUMNS - 1)) ]]; then
      write_report_end_row "$REPORT_FILE"
    fi

    # Increment counter
    i=$(($i + 1))
  done
  
  # Add missing columns to have a complete table
  NUMBER_OF_FRAMEWORKS=${#FRAMEWORKS_REF[@]}
  if [[ $(($NUMBER_OF_FRAMEWORKS % $NUMBER_OF_COLUMNS)) != 0 ]]; then
    REST_COLUMNS=$(($NUMBER_OF_FRAMEWORKS % $NUMBER_OF_COLUMNS))
    MISSING_COLUMNS=$(($NUMBER_OF_COLUMNS - $REST_COLUMNS))
    for COLUMN in $(seq 1 $MISSING_COLUMNS); do
      write_report_empty_column "$REPORT_FILE"
    done
    
    # Finish row
    write_report_end_row "$REPORT_FILE"
  fi
}

# Define test parameters
DURATION=120
TEST_DURATION="$1"
BUILD="$2"
CURRENT_DATE_TIME="`date +%Y%m%d_%H%M%S`";
REPORT_DIR="REPORT_${CURRENT_DATE_TIME}"

if [[ -n "$TEST_DURATION" ]]; then
  DURATION=$TEST_DURATION
fi

printf "Run each test for ${DURATION} seconds\n"

# Add trap to stop running apps if user presses Ctrl+C
trap "../warp/down.sh" INT

# Define list of frameworks to compare
declare -A FRAMEWORKS=()
FRAMEWORKS+=(["1-jvm-micronaut"]="JVM - Micronaut [Kotlin]")
FRAMEWORKS+=(["1-jvm-quarkus"]="JVM - Quarkus [Java]")
FRAMEWORKS+=(["1-jvm-quarkus-vertx"]="JVM - Quarkus Vert.x [Java]")
FRAMEWORKS+=(["1-jvm-spring-mvc"]="JVM - Spring MVC [Java]")
FRAMEWORKS+=(["1-jvm-spring-r2dbc"]="JVM - Spring R2DBC [Java]")
FRAMEWORKS+=(["2-compiled-native-aqueduct"]="Native-Compiled - Aqueduct [Dart]")
FRAMEWORKS+=(["2-compiled-native-hapi"]="Native-Compiled - Hapi [Typescript]")
FRAMEWORKS+=(["2-compiled-native-micronaut"]="Native-Compiled - Micronaut [Kotlin]")
FRAMEWORKS+=(["2-compiled-native-quarkus"]="Native-Compiled - Quarkus [Java]")
FRAMEWORKS+=(["2-compiled-native-quarkus-vertx"]="Native-Compiled - Quarkus Vert.x [Java]")
FRAMEWORKS+=(["3-native-actix"]="Native - Actix [Rust]")
FRAMEWORKS+=(["3-native-actix-sqlx"]="Native - Actix-Sqlx [Rust]")
FRAMEWORKS+=(["3-native-fasthttp"]="Native - Fasthttp [Go]")
FRAMEWORKS+=(["3-native-rocket"]="Native - Rocket [Rust]")
FRAMEWORKS+=(["3-native-warp"]="Native - Warp [Rust]")

declare -A FRAMEWORK_START_CMDS=()
FRAMEWORK_START_CMDS+=(["1-jvm-micronaut"]="up-jvm.sh")
FRAMEWORK_START_CMDS+=(["1-jvm-quarkus"]="up-jvm.sh")
FRAMEWORK_START_CMDS+=(["1-jvm-quarkus-vertx"]="up-jvm.sh")
FRAMEWORK_START_CMDS+=(["1-jvm-spring-mvc"]="up.sh")
FRAMEWORK_START_CMDS+=(["1-jvm-spring-r2dbc"]="up.sh")
FRAMEWORK_START_CMDS+=(["2-compiled-native-aqueduct"]="up-native.sh")
FRAMEWORK_START_CMDS+=(["2-compiled-native-hapi"]="up.sh")
FRAMEWORK_START_CMDS+=(["2-compiled-native-micronaut"]="up-native.sh")
FRAMEWORK_START_CMDS+=(["2-compiled-native-quarkus"]="up-native.sh")
FRAMEWORK_START_CMDS+=(["2-compiled-native-quarkus-vertx"]="up-native.sh")
FRAMEWORK_START_CMDS+=(["3-native-actix"]="up.sh")
FRAMEWORK_START_CMDS+=(["3-native-actix-sqlx"]="up.sh")
FRAMEWORK_START_CMDS+=(["3-native-fasthttp"]="up.sh")
FRAMEWORK_START_CMDS+=(["3-native-rocket"]="up.sh")
FRAMEWORK_START_CMDS+=(["3-native-warp"]="up.sh")

declare -A FRAMEWORK_BUILD_CMDS=()
FRAMEWORK_BUILD_CMDS+=(["1-jvm-micronaut"]="build-jvm.sh")
FRAMEWORK_BUILD_CMDS+=(["1-jvm-quarkus"]="build-jvm.sh")
FRAMEWORK_BUILD_CMDS+=(["1-jvm-quarkus-vertx"]="build-jvm.sh")
FRAMEWORK_BUILD_CMDS+=(["1-jvm-spring-mvc"]="build.sh")
FRAMEWORK_BUILD_CMDS+=(["1-jvm-spring-r2dbc"]="build.sh")
FRAMEWORK_BUILD_CMDS+=(["2-compiled-native-aqueduct"]="build-native.sh")
FRAMEWORK_BUILD_CMDS+=(["2-compiled-native-hapi"]="build.sh")
FRAMEWORK_BUILD_CMDS+=(["2-compiled-native-micronaut"]="build-native.sh")
FRAMEWORK_BUILD_CMDS+=(["2-compiled-native-quarkus"]="build-native.sh")
FRAMEWORK_BUILD_CMDS+=(["2-compiled-native-quarkus-vertx"]="build-native.sh")
FRAMEWORK_BUILD_CMDS+=(["3-native-actix"]="build.sh")
FRAMEWORK_BUILD_CMDS+=(["3-native-actix-sqlx"]="build.sh")
FRAMEWORK_BUILD_CMDS+=(["3-native-fasthttp"]="build.sh")
FRAMEWORK_BUILD_CMDS+=(["3-native-rocket"]="build.sh")
FRAMEWORK_BUILD_CMDS+=(["3-native-warp"]="build.sh")

declare -A FRAMEWORK_PRE_BUILD_CMDS=()
FRAMEWORK_PRE_BUILD_CMDS+=(["3-native-actix"]="rustup default stable")
FRAMEWORK_PRE_BUILD_CMDS+=(["3-native-actix-sqlx"]="rustup default stable")
FRAMEWORK_PRE_BUILD_CMDS+=(["3-native-rocket"]="rustup default nightly")
FRAMEWORK_PRE_BUILD_CMDS+=(["3-native-warp"]="rustup default nightly")

declare -A FRAMEWORK_DIRS=()
FRAMEWORK_DIRS+=(["1-jvm-micronaut"]="micronaut")
FRAMEWORK_DIRS+=(["1-jvm-quarkus"]="quarkus")
FRAMEWORK_DIRS+=(["1-jvm-quarkus-vertx"]="quarkus-vertx")
FRAMEWORK_DIRS+=(["1-jvm-spring-mvc"]="spring-mvc")
FRAMEWORK_DIRS+=(["1-jvm-spring-r2dbc"]="spring-r2dbc")
FRAMEWORK_DIRS+=(["2-compiled-native-aqueduct"]="aqueduct")
FRAMEWORK_DIRS+=(["2-compiled-native-hapi"]="hapi")
FRAMEWORK_DIRS+=(["2-compiled-native-micronaut"]="micronaut")
FRAMEWORK_DIRS+=(["2-compiled-native-quarkus"]="quarkus")
FRAMEWORK_DIRS+=(["2-compiled-native-quarkus-vertx"]="quarkus-vertx")
FRAMEWORK_DIRS+=(["3-native-actix"]="actix-sqlx")
FRAMEWORK_DIRS+=(["3-native-actix-sqlx"]="actix-sqlx")
FRAMEWORK_DIRS+=(["3-native-fasthttp"]="fasthttp")
FRAMEWORK_DIRS+=(["3-native-rocket"]="rocket")
FRAMEWORK_DIRS+=(["3-native-warp"]="warp")

# Run load tests
for FRAMEWORK in "${!FRAMEWORKS[@]}"; do
  echo
  echo "Prepare for ${FRAMEWORK}"
  echo
  reset_db
  cd ../${FRAMEWORK_DIRS[$FRAMEWORK]}
    start_framework ${FRAMEWORK_DIRS[$FRAMEWORK]} ${FRAMEWORK_START_CMDS[$FRAMEWORK]} ${FRAMEWORK_BUILD_CMDS[$FRAMEWORK]} "${FRAMEWORK_PRE_BUILD_CMDS[$FRAMEWORK]}" $BUILD
  cd -
  
  run_load_test $DURATION "${CURRENT_DATE_TIME}_${FRAMEWORK}" "${FRAMEWORKS[$FRAMEWORK]}"
  
  cd ../${FRAMEWORK_DIRS[$FRAMEWORK]}
    stop_framework
  cd -
done

# Generate aggregated report
mkdir -p "${REPORT_DIR}/img"
REPORT_FILE_NAME="report.html"
REPORT_FILE="${REPORT_DIR}/${REPORT_FILE_NAME}"

echo "Generate report"
write_report_header "$REPORT_FILE"

write_report_section FRAMEWORKS "$REPORT_FILE" "$CURRENT_DATE_TIME" "hits-per-second.png" "Hits per second" 2
write_report_section FRAMEWORKS "$REPORT_FILE" "$CURRENT_DATE_TIME" "response-time-over-time.png" "Response time over time" 2
write_report_section FRAMEWORKS "$REPORT_FILE" "$CURRENT_DATE_TIME" "resources-plot.png" "Resources over time" 2
write_report_section FRAMEWORKS "$REPORT_FILE" "$CURRENT_DATE_TIME" "transactions-per-second.png" "Transactions per second" 2
write_report_section FRAMEWORKS "$REPORT_FILE" "$CURRENT_DATE_TIME" "response-time-percentiles.png" "Response time percentiles" 2
write_report_section FRAMEWORKS "$REPORT_FILE" "$CURRENT_DATE_TIME" "bytes-over-time.png" "Bytes over time" 2
write_report_section FRAMEWORKS "$REPORT_FILE" "$CURRENT_DATE_TIME" "codes-per-second.png" "HTTP status codes per second" 2

write_report_footer "$REPORT_FILE"

echo "Finished"
