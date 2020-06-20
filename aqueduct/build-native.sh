#!/bin/bash
aqueduct build
docker build -f Dockerfile.native -t footballmanager-aqueduct-native .
rm footballmanager.aot