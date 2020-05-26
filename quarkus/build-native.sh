#!/bin/bash
./gradlew clean build -Dquarkus.package.type=native
docker build -f src/main/docker/Dockerfile.native -t footballmanager-quarkus-native .
