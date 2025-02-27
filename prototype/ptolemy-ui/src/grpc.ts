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

export const processData = (data: Uint16Array[]): JSON => {
    // TODO - implement this somehow cc: @besaleli
}

export const fetchData = async (client: QueryEngineClient, queryId: string): Promise<Uint8Array[]> => {
    const batches: Uint8Array[] = []
    const call = client.fetchBatch({ queryId })
    for await (const message of call.responses) {
        console.log("newData", message.data)
        batches.push(message.data)
        if (message.isLastBatch) break
    }

    await call.status
    await call.trailers
    console.log(batches, "final table")
    if (!batches.length) {
        throw 'gRPCError: Did not get any data from API!'
    }
    return processData(batches)
}

export async function runQueryAndGetData(query: string): Promise<Table> {
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
