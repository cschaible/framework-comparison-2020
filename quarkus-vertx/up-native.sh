#!/bin/bash
docker run --net docker_default -d -m 256m --cpus 1 -p 8080:8080 --name footballmanager -e QUARKUS_DATASOURCE_JDBC_URL="jdbc:postgresql://postgres:5432/footballmanager" -e QUARKUS_DATASOURCE_REACTIVE_URL="vertx-reactive:postgresql://postgres:5432/footballmanager" footballmanager-quarkus-vertx-native
