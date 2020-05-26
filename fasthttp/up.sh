#!/bin/bash
docker run --net docker_default -d -m 32m --cpus 1 -p 8080:8080 --name footballmanager -e APP_ENV="docker" footballmanager-fasthttp
