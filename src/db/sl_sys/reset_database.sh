#!/bin/bash

set -a
source ./.env
set +a

diesel db reset --database-url "$DATABASE_URL"
