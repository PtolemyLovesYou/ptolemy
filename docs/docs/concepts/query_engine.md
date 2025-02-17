# Overview
Modern observability data often requires complex analysis that goes beyond simple search filters. Ptolemy addresses this need by providing a SQL interface for direct querying - leveraging a language most engineers already know and trust.

The query engine is powered by [DuckDB](https://duckdb.org), an in-process OLAP database chosen for its exceptional performance, extensibility, and ease of use. Running in dedicated containers, it supports optional query logging and data access auditing.

This SQL interface is ideal for ad-hoc queries, simple analytics tasks, and lightweight dashboards. For more demanding use cases that require high reliability, large-scale querying, or complex DAG-based transformations, we recommend either:

- Connecting your Postgres database to a data warehouse
- Building a custom query layer directly on top of Postgres

We recognize that engineers often have strong preferences about their analytics tooling. That's why we've made integration with external tools a priority - our philosophy is to make Ptolemy as flexible and interoperable as possible.
Looking ahead, we're focused on making your observability data even more accessible. We're developing data streaming capabilities to support integrations with message brokers like Kafka and GCP Pub/Sub, ensuring your data can flow seamlessly to where it's needed most.

To learn more about the query engine's service architecture and technical specifications, you can visit [Query Engine Service Architecture](../api_reference/system_diagrams/query_engine.md) in the API reference.
