# Data Model

Ptolemy uses a hierarchical data model designed to capture comprehensive information about machine learning systems at various levels of granularity. This structure enables detailed tracking, debugging, and analysis of ML workflows across environments.

### Hierarchical Structure 🏗️

The platform organizes observability data across four tiers:

1. **System**: The highest level, representing the entire ML application or workflow
2. **Subsystem**: Major functional units within a system
3. **Component**: Individual modules or services within subsystems
4. **Subcomponent**: The smallest trackable units within components

This hierarchical approach allows for both broad system-level insights and detailed component-level analysis.

### Data Categories

Within each tier, the platform captures six types of information:

#### 1. Events ⚡

Events represent executions or actions at each tier. Each event includes:

- A unique identifier
- Name and version
- Parameter configurations (as JSON)
- Environment context (DEV, STAGE, PROD, etc.)

Events form the backbone of the observability model, with each tier's events linked to its parent tier through references.

#### 2. Runtime Information ⏱️

Runtime captures execution details including:

- Start and end timestamps (with microsecond precision)
- Error information (type and content when applicable)
- Associated tier and event reference

This data enables performance tracking, failure analysis, and SLA monitoring.

#### 3. Data Flow: Inputs, Outputs, and Feedback 🔄

Ptolemy systematically tracks the flow of data through your ML systems:

**Inputs**

- Field names and typed values (supporting string, integer, float, boolean, and JSON)
- Input context and metadata
- Enables reproducibility and helps identify how varying inputs affect outcomes

**Outputs**

- Results produced at each tier using the same flexible data typing system
- Captures various return formats consistently

**Feedback**

- Auxiliary metrics collected during or immediately after execution
- Includes quality scores, toxicity measurements, compliance metrics, and other immediate evaluations
- Enables real-time quality assessment of model performance

This three-part data flow tracking creates a complete picture of how information transforms throughout your ML pipeline.

#### 4. Metadata 🏷️

Metadata provides additional context through string key-value pairs, useful for:

- Tagging executions
- Adding identifiers
- Including searchable annotations
- Linking to external systems

!!! example "Conversation Turn Example: Hierarchical Breakdown"
    Let's examine how a single turn of dialogue in a conversational AI system would be tracked across Ptolemy's hierarchical levels:

    ```
    ├── System Level: Conversation Manager 🗣️
        │   ├── Event: dialogue_turn_processed
        │   ├── Inputs: User query text, conversation history
        │   ├── Outputs: Complete system response
        │   ├── Feedback: Overall response quality score, user satisfaction rating
        │   └── Metadata: user_id, conversation_id, session_id, client_application
        │
        ├── Subsystem Level: Natural Language Understanding 🧠
        │   ├── Event: query_intent_classified
        │   ├── Inputs: User query text
        │   ├── Outputs: Intent classification, entity extraction results, confidence scores
        │   ├── Feedback: Intent classification accuracy
        │   └── Metadata: request_id, model_version, language_detected
        │
        ├── Subsystem Level: Retrieval-Augmented Generation (RAG) 📚
        │   ├── Event: context_augmentation_executed
        │   ├── Inputs: Processed query, knowledge base parameters
        │   ├── Outputs: Retrieved context passages, relevance scores
        │   ├── Feedback: Retrieval precision/recall metrics
        │   ├── Metadata: knowledge_base_id, vector_index_version, retrieval_strategy
        │   │
        │   ├── Component Level: Embedding Generation 🔤
        │   │   ├── Event: query_embedding_created
        │   │   ├── Inputs: Processed query text
        │   │   ├── Outputs: Vector embedding
        │   │   ├── Feedback: Embedding quality metrics
        │   │   └── Metadata: embedding_model_id, embedding_dimensions, normalization_applied
        │   │
        │   └── Component Level: Vector Search 🔍
        │       ├── Event: vector_similarity_search_executed
        │       ├── Inputs: Query embedding, search parameters
        │       ├── Outputs: Top k matching documents with similarity scores
        │       ├── Feedback: Search latency, cache hit rate
        │       └── Metadata: collection_name, index_id, search_algorithm, cache_used
        │
        ├── Subsystem Level: Response Generation ✍️
        │   ├── Event: llm_response_generated
        │   ├── Inputs: Processed query, retrieved context, conversation history, system prompt
        │   ├── Outputs: Raw LLM response
        │   ├── Feedback: Generation quality metrics, toxicity scores, hallucination detection
        │   ├── Metadata: llm_model_id, temperature, prompt_tokens, completion_tokens
        │   │
        │   ├── Component Level: Prompt Construction 📝
        │   │   ├── Event: prompt_assembled
        │   │   ├── Inputs: Template variables
        │   │   ├── Outputs: Constructed prompt text
        │   │   ├── Feedback: Token count, prompt complexity score
        │   │   ├── Metadata: template_id, template_version, jinja_used, prompt_strategy
        │   │   │
        │   │   └── Subcomponent Level: Context Truncation ✂️
        │   │       ├── Event: context_truncated
        │   │       ├── Inputs: Retrieved passages, token limit
        │   │       ├── Outputs: Truncated context
        │   │       ├── Feedback: Information preservation score
        │   │       └── Metadata: truncation_strategy, total_passages, passages_used
        │   │
        └── Subsystem Level: Response Formatting 🎨
            ├── Event: response_postprocessed
            ├── Inputs: Raw LLM output
            ├── Outputs: Formatted response
            ├── Feedback: Formatting quality checks
            └── Metadata: formatter_id, output_format, post_processing_steps

    ```

    This hierarchical tracking enables:

    - Pinpointing exactly where issues occur (e.g., poor retrieval vs. generation problems)
    - Understanding performance bottlenecks across the entire conversation flow
    - Correlating user satisfaction with specific component behaviors
    - Debugging complex interactions between subsystems
    - Tracing requests through the entire system with consistent metadata identifiers

    In this case, as well as in many other cases, you may notice that subcomponents aren't used. In the interest of keeping things simple, only using the amount of required tiers is considered good practice.


### Data Type Flexibility 🧩

A core principle of Ptolemy's design is the flexible handling of input, output, and feedback data. This flexibility is critical for ML observability due to the diverse nature of machine learning workloads:

1. **Polymorphic Data Storage**: Ptolemy stores values in type-specific fields (string, integer, float, boolean, or JSON) while maintaining a unified query interface.

2. **JSON Support for Complex Structures**: For nested or complex data formats like prompt templates, embedding vectors, or configuration objects, the JSON type provides unlimited flexibility without requiring schema modifications.

3. **Type Safety with Runtime Flexibility**: The `field_value_type` enum ensures type safety while allowing for dynamic data handling, enabling Ptolemy to adapt to various ML frameworks and model types without code changes.

4. **Single Field Conceptual Model**: Although implemented as separate columns for efficiency, conceptually each field represents a single value that can be of any supported type, simplifying the developer experience.

5. **Cross-Framework Compatibility**: This approach enables Ptolemy to accommodate diverse ML ecosystems, from traditional statistical models to neural networks to large language models, each with their own input/output characteristics.

This flexible type system is particularly valuable for:
- LLM applications with text inputs/outputs alongside numerical configuration parameters
- Multimodal models that process various data types
- Ensemble systems combining different model architectures
- Feature stores with heterogeneous feature types
- Experimental workflows where data schemas evolve frequently

## Data Management 🗄️

The platform implements soft deletion throughout the data model. Rather than permanently removing records, the system:

1. Marks records with deletion timestamps
2. Records deletion reasons
3. Preserves the data for audit and analysis purposes

This approach maintains data lineage and enables historical analysis while supporting data governance requirements.

## Schema Design Principles 📐

The data model follows several key design principles:

1. **Referential Integrity**: Cascading deletes ensure that related records remain consistent
2. **Type Safety**: Enumerated types enforce data validation
3. **Flexible Value Storage**: Different data types are accommodated through type-specific fields
4. **Constraint Enforcement**: Check constraints ensure that records are associated with the correct tier

## Systems Engineering Alignment 🔧

Ptolemy's data model is deliberately structured to align with traditional systems engineering principles:

### Hierarchical Decomposition

The four-tier structure (system, subsystem, component, subcomponent) directly mirrors the classic systems engineering approach of breaking down complex systems into manageable, functionally distinct parts. This decomposition:

1. **Enables Clear Boundaries**: Each tier has well-defined responsibilities and interfaces
2. **Supports Modularity**: Changes in one component can be isolated without affecting others
3. **Facilitates Traceability**: Issues can be tracked through the hierarchy to their source
4. **Promotes Reusability**: Well-defined components can be reused across different systems

### Separation of Concerns

Ptolemy enforces good system architecture by separating different aspects of ML workflows:

1. **Configuration vs. Execution**: Parameters are separated from runtime information
2. **Functional Logic vs. Performance**: Events capture what happened, while runtime tracks how efficiently it occurred
3. **Data Flow Transparency**: Explicit tracking of inputs and outputs makes data lineage clear
4. **Metadata Independence**: Contextual information is kept separate from functional data

### System Boundaries and Interfaces

The data model explicitly captures system interfaces through:

1. **Defined Input/Output Contracts**: Each tier's inputs and outputs are formally recorded
2. **Clear Parent-Child Relationships**: References between tiers enforce proper hierarchical structure
3. **Environment Context**: The environment field ensures proper separation between development, staging, and production

### Governance and Quality Assurance

Ptolemy's model embeds governance principles:

1. **Soft Deletion**: Maintains audit trails and historical context
2. **Version Tracking**: Captures evolutionary changes in systems
3. **Error Documentation**: Explicitly tracks failure modes and error types
4. **Feedback Integration**: Incorporates quality metrics directly into the observability framework

### Adaptability and Evolution

The flexible type system ensures that Ptolemy can evolve alongside ML technology:

1. **Future-Proofing**: New model types can be integrated without schema changes
2. **Progressive Enhancement**: Systems can begin with simple metrics and add complexity over time
3. **Technology Independence**: The data model makes no assumptions about specific ML frameworks

By adhering to these systems engineering principles, Ptolemy not only provides observability but also gently guides organizations toward better ML system architecture. The very act of instrumenting ML systems with Ptolemy encourages developers to think systematically about system boundaries, interfaces, and component responsibilities - leading to more maintainable, debuggable, and robust ML applications.

To learn more about Ptolemy's data model, check out our [System Diagrams](../api_reference/system_diagrams/database_schema.md).
