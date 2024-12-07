#!/bin/bash

set -a
source ./.env
set +a

diesel migration run --database-url "$DATABASE_URL"