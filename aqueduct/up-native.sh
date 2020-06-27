#!/bin/bash
$(dirname "$0")/migrate.sh
docker run --net docker_default -d -m 384m --cpus 1 -p 8080:80 --name footballmanager -e DATABASE_URL="postgres://footballmanager:secret@postgres:5432/footballmanager" footballmanager-aqueduct-native
