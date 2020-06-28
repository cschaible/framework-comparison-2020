#!/bin/bash
./gradlew clean build
cd build/libs
# Add reflection config for PG reflection issue: https://github.com/oracle/graal/issues/2195
native-image --no-server -cp footballmanager-*-all.jar -H:ReflectionConfigurationFiles=../../reflectconfig
cd -
docker build --build-arg "APP_FILE=build/libs/footballmanager" -f Dockerfile.native -t footballmanager-micronaut-native .
