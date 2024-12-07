#!/bin/bash

set -a
source ./.env
set +a

diesel migration run --config-file diesel.toml --database-url "$DATABASE_URL"
diesel migration run --config-file diesel.toml --database-url "$DATABASE_URL_TEST"