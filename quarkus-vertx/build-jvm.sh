#!/bin/bash
./gradlew clean build
docker build -f src/main/docker/Dockerfile.jvm -t footballmanager-quarkus-vertx-jvm .
