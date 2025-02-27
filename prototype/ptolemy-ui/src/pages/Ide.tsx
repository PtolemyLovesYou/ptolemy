import { useState } from "react"

import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"


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
            <Button onClick={() => alert('Not implemented yet!')}>Search</Button>
        </div>
    );
}
export default IDE
