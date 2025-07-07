# Services Configuration

The API component handles all external requests and manages authentication. Its configuration is divided into two main sections: general settings and Postgres database configurations.

## General Configuration
These settings control core API behaviors, including authentication and auditing. The sysadmin credentials are particularly important as they provide initial system access. The JWT secret is used for secure token generation and should be a strong, randomly generated value.

{{ read_json('tables/api_config.json', orient='records') }}

## Postgres Configuration
Ptolemy uses Postgres as its primary datastore. These settings define how the API connects to your Postgres instance. While default values are provided for most settings, in production environments you should explicitly configure all parameters to ensure security and reliability.

{{ read_json('tables/postgres_config.json', orient='records') }}
