-- +goose Up
-- +goose StatementBegin
CREATE TABLE workspace (
    id UUID PRIMARY KEY,
    name varchar(64),
    created_at timestamp,
    updated_at timestamp
    );
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE workspace;
-- +goose StatementEnd
