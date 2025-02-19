# Services Configuration

Ptolemy's configuration is split into two main components: the API service and the Query Engine. Each component requires specific configuration to ensure proper operation and security. Below you'll find detailed configuration tables along with explanations for each section.

## API Configuration
The API component handles all external requests and manages authentication. Its configuration is divided into three main sections: general settings, Postgres database configuration, and Redis caching configuration.

### General Configuration
These settings control core API behaviors, including authentication and auditing. The sysadmin credentials are particularly important as they provide initial system access. The JWT secret is used for secure token generation and should be a strong, randomly generated value.

{{ read_json('tables/api_config.json', orient='records') }}

### Postgres Configuration
Ptolemy uses Postgres as its primary datastore. These settings define how the API connects to your Postgres instance. While default values are provided for most settings, in production environments you should explicitly configure all parameters to ensure security and reliability.

{{ read_json('tables/postgres_config.json', orient='records') }}

### Redis Configuration
Redis is used for as a message broker between services These settings configure the Redis connection. While Redis authentication is optional, it's recommended in production environments.

{{ read_json('tables/redis_config.json', orient='records') }}

!!! warning "Security Considerations"
    When deploying Ptolemy, pay special attention to security configuration:

    - Always use strong, unique passwords for all required credentials
    - Generate a secure random string for the JWT secret
    - Consider enabling auditing in production environments
    - Use appropriate network security measures to protect database and Redis instances
    - Review and customize default values for your specific environment

    The configuration options below provide flexibility for both development and production deployments while maintaining security best practices.

# Query Engine Configuration
The Query Engine processes analysis requests asynchronously using Redis streams. Its configuration mirrors the API configuration in many ways, but includes additional settings for stream processing.

### General Configuration
These settings control how the Query Engine processes requests. The stream and group names define how analysis jobs are organized and processed in Redis.

{{ read_json('tables/query_engine_config.json', orient='records') }}

### Postgres Configuration
The Query Engine needs its own connection to Postgres to read and write analysis results. These settings match the API's Postgres configuration format.

{{ read_json('tables/postgres_config.json', orient='records') }}

### Redis Configuration
Like the API, the Query Engine needs Redis access for job management and caching. These settings match the API's Redis configuration.

{{ read_json('tables/redis_config.json', orient='records') }}
    