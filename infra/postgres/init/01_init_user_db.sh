#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE USER mywork;
    CREATE DATABASE myworkdb;
    GRANT ALL PRIVILEGES ON DATABASE myworkdb TO mywork;
EOSQL
