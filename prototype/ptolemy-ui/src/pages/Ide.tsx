import { useState } from "react"

import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"

import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport"
import { QueryEngineClient } from "@/generated/query_engine.client"


interface DataProps {
    query: string
}
async function Data({ query }: DataProps) {
    const transport = new GrpcWebFetchTransport({ baseUrl: import.meta.env.VITE_PTOLEMY_API})
    const client = new QueryEngineClient(transport);

    const { response: { queryId, error, success } } = await client.query({ query })
    if (error) return <pre>{error}</pre>
    if (success) {
        const array = []
        const call = client.fetchBatch({ queryId })
        for await (const message of call.responses) {
            array.concat(<pre>{message.data.toString()}</pre>)
        }
        return array
    }
    // if (loading) return "Loading...";

    // return (
    //     <pre>{data}</pre>
    // )
}

function IDE() {
    const [query, setQuery] = useState("");
    const [input, setInput] = useState("");

    return (
        <div>
            <h1>IDE</h1>
            <Textarea
                className="w-lg my-5"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                placeholder="Type your GraphQL query here."
            />
            <Button onClick={() => setQuery(input)}>Search</Button>
            <ul>
                {query ? <Data query={query} /> : null}
            </ul>
        </div>
    );
}
export default IDE
