#!/bin/bash
./gradlew clean build
docker build --build-arg "JAR_FILE=build/libs/footballmanager-0.1-all.jar" -f Dockerfile.jvm -t footballmanager-micronaut-jvm .
