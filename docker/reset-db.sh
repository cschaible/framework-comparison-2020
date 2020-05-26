#!/bin/bash

docker exec postgres psql -U footballmanager -d postgres -c "DROP DATABASE footballmanager;"
docker exec postgres psql -U footballmanager -d postgres -c "CREATE DATABASE footballmanager;"
