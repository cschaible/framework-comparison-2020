#!/bin/bash
docker run --net docker_default -d -m 256m --cpus 1 -p 8080:8080 --name footballmanager -e SPRING_DATASOURCE_URL="jdbc:postgresql://postgres:5432/footballmanager?user=footballmanager&password=secret" footballmanager-spring-r2dbc
