if [[ $DB = "clickhouse" ]]; then
    echo "Setting driver to clickhouse" && \
        export GOOSE_DRIVER="clickhouse" && \
        export GOOSE_DBSTRING="http://clickhouse:8123/?enable_json_type=1&enable_variant_type=1" && \
        export GOOSE_MIGRATION_DIR="clickhouse";
else
    echo "Invalid DB: $DB" && exit 1;
fi
