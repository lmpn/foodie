#!/bin/bash
DB_URL=${DB_URL:-sqlite:sql/test.db}

sqlx db create --database-url "$DB_URL"
sqlx migrate run --source sql/migrations --database-url "$DB_URL"