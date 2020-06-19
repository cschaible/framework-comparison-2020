#!/bin/bash
docker run --net docker_default -d -m 128m --cpus 1 -p 8080:8080 -e NODE_OPTIONS=128 --name footballmanager footballmanager-hapi
