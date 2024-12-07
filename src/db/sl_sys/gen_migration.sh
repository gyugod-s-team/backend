#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <migration_name>"
    exit 1
fi

MIGRATION_NAME=$1

diesel migration generate $MIGRATION_NAME