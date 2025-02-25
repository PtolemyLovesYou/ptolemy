import { useState } from "react"
import { gql, useQuery } from "@apollo/client"
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"


interface DataProps {
    query: string
}
function Data({ query }: DataProps) {
    const { data, loading, error } = useQuery(gql`${query}`);
    if (loading) return "Loading...";
    if (error) return <pre>{error.message}</pre>
    return (
        <pre>{data}</pre>
    )
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
