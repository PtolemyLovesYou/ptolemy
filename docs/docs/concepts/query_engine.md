# Query Engine
Modern observability data often requires complex analysis that goes beyond simple search filters. Ptolemy addresses this need by providing a SQL interface for direct querying - leveraging a language most engineers already know and trust.

The query engine is powered by [DuckDB](https://duckdb.org), an in-process OLAP database chosen for its exceptional performance, extensibility, and ease of use. Running in dedicated containers, it supports optional query logging and data access auditing.

This SQL interface is ideal for ad-hoc queries, simple analytics tasks, and lightweight dashboards. For more demanding use cases that require high reliability, large-scale querying, or complex DAG-based transformations, we recommend either:

- Connecting your Postgres database to a data warehouse
- Building a custom query layer directly on top of Postgres

We recognize that engineers often have strong preferences about their analytics tooling. That's why we've made integration with external tools a priority - our philosophy is to make Ptolemy as flexible and interoperable as possible.
Looking ahead, we're focused on making your observability data even more accessible. We're developing data streaming capabilities to support integrations with message brokers like Kafka and GCP Pub/Sub, ensuring your data can flow seamlessly to where it's needed most.

!!! question ":lock: Compliance & Security"
    Balancing data security with powerful querying shouldn't require a full data warehouse setup. We've built a security framework for Ptolemy's query engine that keeps things simple while ensuring your data stays safe through:

    - Fine-grained application-layer access controls to help you manage who sees what
    - Custom Postgres permission schemes that make it easy to set boundaries
    - Configurable resource quotas to keep everything running smoothly
    - Network and filesystem isolation to keep your queries in their own secure space

    We've designed Ptolemy with security and compliance in mind, but we know there's always room for improvement! If you're passionate about cybersecurity or data governance, we'd love to hear your thoughts - check out our contribution guidelines to learn how you can help make Ptolemy even better.

The query engine is an optional feature and can be disabled. To learn more, visit [Configuration](../api_reference/configuration.md).

To learn more about the query engine's service architecture and technical specifications, visit [Query Engine Service Architecture](../api_reference/system_diagrams/query_engine.md).
