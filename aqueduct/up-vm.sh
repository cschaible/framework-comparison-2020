#!/bin/bash
./migrate.sh
docker run --net docker_default -d -m 768m --cpus 1 -p 8080:80 --name footballmanager -e DATABASE_URL="postgres://footballmanager:secret@postgres:5432/footballmanager" footballmanager-aqueduct-vm
