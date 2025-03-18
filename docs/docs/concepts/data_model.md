# Data Model

Ptolemy uses a hierarchical data model designed to capture comprehensive information about machine learning systems at various levels of granularity. This structure enables detailed tracking, debugging, and analysis of ML workflows across environments.

### Hierarchical Structure 🏗️

The platform organizes observability data across four tiers:

1. **System**: The highest level, representing the entire ML application or workflow
2. **Subsystem**: Major functional units within a system
3. **Component**: Individual modules or services within subsystems
4. **Subcomponent**: The smallest trackable units within components

This hierarchical approach allows for both broad system-level insights and detailed component-level analysis.

??? guide "Structuring Your ML System in Ptolemy"
    **Tiers**

    | Tier | What It Represents | How to Identify | Example |
    |------|-------------------|----------------|---------|
    | **System** | Complete ML application | Has its own API, solves a business problem | Recommendation Engine |
    | **Subsystem** | Major functional area | Distinct processing phase, owned by specific team | Candidate Generation |
    | **Component** | Single-purpose unit | Specific algorithm, clear inputs/outputs | Vector Search |
    | **Subcomponent** | Algorithm step | Use sparingly for complex components needing detailed monitoring | Query Tokenization |

    **Best Practices:**

    - Start with Systems and Subsystems, add deeper tiers as needed
    - Aim for 3-7 Subsystems per System
    - Ensure data flows logically between tiers
    - Use consistent granularity for similar functionality

    **Common Pattern Example:**
    ```
    ├── System: ML Service
        ├── Subsystem: Data Processing
        ├── Subsystem: Model Inference
        │   ├── Component: Algorithm A
        │   └── Component: Algorithm B
        └── Subsystem: Post-Processing
    ```

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

!!! note "Why Four Tiers? Understanding Supersystems in Ptolemy"
    **The Four-Tier Architecture**

    Ptolemy intentionally limits its hierarchy to four tiers (System, Subsystem, Component, Subcomponent) to balance observability with practical usability. But what about higher-level constructs?

    **Supersystems: The Fifth Tier That Isn't**

    *Supersystems* represent end-to-end workflows that span multiple systems. For example:

    - A complete user conversation spanning multiple turns
    - A multi-stage ML pipeline crossing service boundaries
    - A business process involving several ML systems

    Rather than adding a fifth tier, Ptolemy recommends using *metadata* to track supersystem relationships:

    ```
    # Instead of:
    ├── Supersystem: User Conversation
        └── System: Turn Processing

    # Use metadata at the System level:
    ├── System: Turn Processing
        └── Metadata: conversation_id=abc123, turn_number=3
    ```

    **Why This Approach?**

    1. *Simplicity*: Four tiers provide sufficient granularity without overwhelming complexity
    2. *Query Flexibility*: Metadata-based grouping enables more dynamic supersystem analysis
    3. *Cross-Cutting Concerns*: Some systems may participate in multiple supersystems
    4. *Varying Lifecycles*: Supersystems often have different retention and governance needs

    This approach gives you supersystem visibility while keeping it simple, stupid.

#### 4. Metadata 🏷️

Metadata provides additional context through string key-value pairs, useful for:

- Tagging executions
- Adding identifiers
- Including searchable annotations
- Linking to external systems

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
