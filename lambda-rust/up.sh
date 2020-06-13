#!/bin/bash

sam local start-api --port 8080 --skip-pull-image --env-vars env.json --docker-network=docker_default
