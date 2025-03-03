import { useEffect, useState } from 'react';

import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { runQueryAndGetData } from '@/grpc';
import { Loader2 } from 'lucide-react';

interface DataProps {
  query: string;
  setIsLoading: (isLoading: boolean) => void;
}

function Data({ query, setIsLoading }: DataProps) {
  const [schema, setSchema] = useState<string[]>([]);
  const [data, setData] = useState<string[][]>([]);
  useEffect(() => {
    if (!query) {
      return;
    }
    const fetchData = async () => {
      const [schema, data] = await runQueryAndGetData(query);
      setSchema(schema);
      setData(data);
      setIsLoading(false);
    };
    fetchData();
  }, [query, setIsLoading]);

  return (
    <Table>
      <TableHeader>
        <TableRow>
          {schema.map((field, i) => (
            <TableHead key={i}>{field}</TableHead>
          ))}
        </TableRow>
      </TableHeader>
      <TableBody>
        {data.map((row, i) => (
          <TableRow key={i}>
            {row.map((cell, j) => (
              <TableCell key={j}>{cell}</TableCell>
            ))}
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}

function IDE() {
  const [query, setQuery] = useState('');
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  return (
    <div>
      <h1>IDE</h1>
      <Textarea
        className='w-lg my-5'
        value={input}
        onChange={(e) => setInput(e.target.value)}
        placeholder='Type your SQL query here.'
      />

      <Button
        onClick={() => {
          setQuery(input);
          if (input !== query) setIsLoading(true);
        }}
        disabled={input === query || isLoading}
      >
        {isLoading ? (
          <>
            <Loader2 className='animate-spin' /> Running...
          </>
        ) : (
          'Run'
        )}
      </Button>
      {}
      {/* TODO handle refetching data (right now, doesn't work so disabled. */}
      {query ? <Data query={query} setIsLoading={setIsLoading} /> : null}
    </div>
  );
}
export default IDE;
