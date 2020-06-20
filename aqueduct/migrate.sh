#!/bin/bash
cat ./migrations/V01__CREATE_FOOTBALLER_TABLE.sql | PGPASSWORD="secret" docker exec -i postgres psql -U footballmanager -d footballmanager
