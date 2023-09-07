#!/bin/bash
DB_URL=${DB_URL:-sqlite:sql/test.db}
sqlx database drop -y --database-url "${DB_URL}"