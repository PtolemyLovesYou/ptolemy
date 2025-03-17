# Ptolemy

## Open Source Universal ML Observability

Ptolemy is an open-source ML monitoring platform built on systems engineering principles that brings clarity to complex ML deployments. Created by engineers who value architectural rigor, performance, and flexibility, Ptolemy delivers comprehensive observability without requiring you to reinvent the wheel for each new methodology.

:rocket: Try out Ptolemy now using our [Getting Started](getting_started/installation_docker_compose.md) guide!

!!! danger "Ptolemy is in Alpha!"
    Ptolemy is still in alpha and we would love your help making it better! See our guide on [Contribution](contributing/index.md) to learn more about submitting feedback and contributing.

## Why Ptolemy?

### :building_construction: Systems-First Design
Ptolemy organizes your ML monitoring across four logical levels (system, subsystem, component, subcomponent) and four data types (input, output, feedback, metadata). This structured yet flexible framework makes troubleshooting and optimization intuitive, even for complex ML deployments. [Learn more about our data model here.](concepts/data_model.md)

### :woman_scientist: Direct SQL Access
Query your data directly with SQL — no black boxes, no limitations. While other platforms lock you into rigid semantic layers, Ptolemy lets you run the complex analyses you actually need, using a language you already know. Learn more about Ptolemy's SQL capabilities in our guide on Ptolemy's [Query Engine](concepts/query_engine.md).

### :zap: Lightning-Fast Performance
Built with Rust and gRPC for speed-critical components, Ptolemy ingests and processes your data with minimal overhead. We've optimized every part of the stack because milliseconds matter when you're debugging production.

### :dart: Nexus, Not a Replacement
We don't want to replace your existing monitoring tools — they're great at what they do. Instead, Ptolemy serves as a central nexus, connecting your observability data sources through our flexible connector ecosystem and extracting insights that would otherwise remain hidden.

### :lock: Security Without Sacrifice
While we're building toward full compliance certifications, Ptolemy already includes comprehensive auditing capabilities, fine-grained access controls, and customizable permission schemes. Your sensitive ML data deserves nothing less.

### :rocket: Deploy Without Drama
We've built Ptolemy on battle-tested technologies you likely already have deployed. No exotic dependencies, no complex infra prerequisites – just standard components that your ops team already knows how to maintain. Deploy in minutes, not months, without fighting the endless approval battles plaguing ML tooling adoption. Because great observability shouldn't require a six-month procurement cycle.

### :jigsaw: Built for Extensibility
We understand that ML engineers are (rightfully) opinionated about their observability needs. Every ML system is unique, which is why Ptolemy prioritizes extensibility at our core. Our plugin architecture lets you integrate with your existing pipelines and tools without forcing you to adopt our opinions. Whether you're using custom metrics, proprietary data formats, or specialized visualization tools, Ptolemy's flexible APIs and connector framework adapt to your workflow – not the other way around.
