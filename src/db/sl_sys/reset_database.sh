#!/bin/bash

set -a
source ./.env
set +a

diesel db reset --config-file diesel.toml --database-url "$DATABASE_URL"
diesel db reset --config-file diesel.toml --database-url "$DATABASE_URL_TEST"
