if [[ $DB = "postgres" ]]; then
    echo "Setting driver to postgres" && \
        export GOOSE_DRIVER="postgres" && \
        export GOOSE_DBSTRING="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST:$POSTGRES_PORT/$POSTGRES_DB" && \
        export GOOSE_MIGRATION_DIR="postgres";
elif [[ $DB = "clickhouse" ]]; then
    echo "Setting driver to clickhouse" && \
        export GOOSE_DRIVER="clickhouse" && \
        export GOOSE_DBSTRING="http://clickhouse:8123/?enable_json_type=1&enable_dynamic_type=1" && \
        export GOOSE_MIGRATION_DIR="clickhouse";
else
    echo "Invalid DB: $DB" && exit 1;
fi
