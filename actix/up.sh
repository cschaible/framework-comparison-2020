#!/bin/bash
./setup_migrate.sh
docker run --net docker_default -d -m 32m --cpus 1 -p 8080:8080 --name footballmanager -e DATABASE_URL="postgres://footballmanager:secret@postgres:5432/footballmanager" footballmanager-actix
