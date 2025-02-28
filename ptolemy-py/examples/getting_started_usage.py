"""Basic usage example."""
import logging
import pandas as pd
import ptolemy_client as pt

# Initialize the client with configuration
client = pt.Ptolemy(
    base_url="http://localhost:8000",
    api_key="your-api-key-here",
    workspace_name="my-workspace",
    autoflush=False,
    batch_size=256
)

# Create a root trace for the main system
try:
    sys = client.trace(
        name="chat-completion",
        version='1.2.3',
        environment='PROD',
        parameters={
            "model": "gpt-4",
            "service": "openai"
        }
    )

    with sys:
        # Log system-level metadata (strings only)
        sys.metadata(
            customer_id="123",
            session_id="789",
            request_id="req_456"
        )

        # Log the initial request inputs
        sys.inputs(
            messages=[
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": "Tell me about quantum computing"}
            ],
            temperature=0.7,
            max_tokens=1000
        )

        # Track the retrieval subsystem
        retriever = sys.child(
            name="document-retriever",
            version='1.2.3',
            environment='PROD',
            parameters={"engine": "elasticsearch"}
        )

        with retriever:
            # Log retrieval inputs
            retriever.inputs(
                query="quantum computing",
                n_results=5,
                filter_criteria={"date_range": "last_year"}
            )

            # Simulate document retrieval
            documents = [
                {"id": "doc1", "title": "Quantum Computing Basics"},
                {"id": "doc2", "title": "Quantum Algorithms"}
            ]

            # Log retrieval outputs
            retriever.outputs(
                documents=documents,
                retrieval_time_ms=150
            )

            # Log any retrieval-specific metadata (strings only)
            retriever.metadata(
                cache_hit="false",
                index_version="v2.1"
            )

        # Log the main system outputs
        completion_content = "Quantum computing is a type of computation..."
        sys.outputs(
            response=completion_content,
            documents=documents,
            token_count=525
        )

        # Log feedback and evaluation metrics
        # Note: It's okay and expected to not log *all* feedback here! With Ptolemy, you
        # can always access external data using the SQL interface.
        sys.feedback(
            relevance_score=0.95,
            toxicity_score=0.02,
            response_quality=0.89
        )

except Exception as e:
    logging.error("Error in chat completion: %s", e)
finally:
    # Ensure we flush any remaining events even if there's an error
    client.flush()

# Example of using the SQL interface
df = pd.concat(client.sql("SELECT * from ptolemy.system_event WHERE name = 'chat-completion'"))
print(df.head())
