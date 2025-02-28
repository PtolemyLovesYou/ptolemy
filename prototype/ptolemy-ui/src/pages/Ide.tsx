import { useEffect, useState } from "react"

import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { runQueryAndGetData } from "@/grpc";

function Data({ query }: { query: string }) {
    const [data, setData] = useState('')
    useEffect(() => {
        if (!query) {
            return
        }
        const fetchData = async () => {
            const result = await runQueryAndGetData(query)
            setData(result)
        }
        fetchData()
    })
    return <pre>{data || 'No data available'}</pre>
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
                placeholder="Type your SQL query here."
            />
            <Button onClick={() => setQuery(input)}>Run</Button>
            {query ? <Data query={query} /> : null}
        </div>
    );
}
export default IDE
