# Configure Diesel env vars
export DATABASE_URL=postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST/$POSTGRES_DB

# Configure Goose env vars for Clickhouse
export GOOSE_DBSTRING="http://clickhouse:8123/?enable_json_type=1&enable_variant_type=1"
