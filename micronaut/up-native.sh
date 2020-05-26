#!/bin/bash
docker run --net docker_default -d -m 256m --cpus 1 -p 8080:8080 --name footballmanager -e MICRONAUT_DATASOURCE_URL="jdbc:postgresql://postgres:5432/footballmanager" footballmanager-micronaut-native
