#!/bin/bash
docker run --net docker_default -d -m 32m --cpus 1 -p 8080:8080 --name footballmanager-warp -e DATABASE_URL="postgres://footballmanager:secret@postgres:5432/footballmanager" footballmanager-warp
