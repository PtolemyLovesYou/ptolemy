import { Table, tableFromIPC } from "@apache-arrow/ts"
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import { QueryEngineClient } from "./generated/query_engine.client";
import { RpcOptions, UnaryCall } from "@protobuf-ts/runtime-rpc";
import { AUTH_TOKEN_KEY } from "./constants";
import { QueryStatus, QueryStatusResponse } from "./generated/query_engine";

export const grpcClient = () => {
        const options: RpcOptions = {
            interceptors: [
              {
                // adds auth header to unary requests
                interceptUnary(next, method, input, options: RpcOptions): UnaryCall {
                  if (!options.meta) {
                    options.meta = {};
                  }
                      options.meta['Authorization'] = `Bearer ${localStorage.getItem(AUTH_TOKEN_KEY) || ''}`;
                      options.meta['grpc-accept-encoding'] = 'identity'
                      options.meta['grpc-encoding'] = 'identity'
                  return next(method, input, options);
                }
              }
            ],
        };
        const transport = new GrpcWebFetchTransport({ baseUrl: import.meta.env.VITE_PTOLEMY_API, ...options})
        return new QueryEngineClient(transport);
}

export const runQuery = async (client: QueryEngineClient, query: string) => {
    const { response: { queryId, success, error } } = await client.query({ query })
    if (success) {
        return queryId
    } else {
        throw error
    }

}

const FINAL_STATES = [QueryStatus.CANCELLED, QueryStatus.COMPLETED, QueryStatus.FAILED]

export const waitForQuery = async (client: QueryEngineClient, queryId: string):
Promise<QueryStatusResponse> => {
    let { response } = await client.getQueryStatus({ queryId })
    while (!FINAL_STATES.includes(response.status)) {
        response = (await client.getQueryStatus({ queryId })).response
    }
    return response
}

const readDataFromRow = (fields, row) => {
    return fields
      .map((_, i) => row.get(i))
      .join(',');
  };

export const processData = (data: Uint8Array[]): string => {
    const table: Table = data.reduce((prev: Table | null, curr: Uint8Array) => {
        if (prev === null) {
            return tableFromIPC(curr)
        } else {
            return prev.concat(tableFromIPC(curr))
        }
    }, null) as Table
    if (table === null) throw 'No data received!'
    const columns = table.schema.fields.map((f) => f.name)
    let dataStr = columns.join(',') + '\n';

    for (let i = 0; i < table.numRows; i++) {
      const rowData = readDataFromRow(columns, table.getChildAt(i));
      dataStr += `${rowData}\n`;
    }

    return dataStr
}

export const fetchData = async (client: QueryEngineClient, queryId: string): Promise<string> => {
    const batches: Uint8Array[] = []
    const call = client.fetchBatch({ queryId })
    for await (const message of call.responses) {
        batches.push(message.data)
        if (message.isLastBatch) break
    }

    await call.status
    await call.trailers
    if (!batches.length) {
        throw 'gRPCError: Did not get any data from API!'
    }
    return processData(batches)
}

export async function runQueryAndGetData(query: string): Promise<string> {
    const client = grpcClient()
    const table = await runQuery(client, query).then(async (queryId) => {
        return await waitForQuery(client, queryId).then(
            async ({ status, error }) => {
                switch (status) {
                    case QueryStatus.COMPLETED:
                        return await fetchData(client, queryId)
                    case QueryStatus.CANCELLED:
                        throw 'Query was Canceled'
                    case QueryStatus.FAILED:
                    default:
                        throw error || 'Unknown error'


                }
            }
        )
    })
    return table
}

// query -> queryId ; waitForQuery(queryId) -> QueryStatus.COMPLETE; getQuery({ queryId }) -> Data or Error
