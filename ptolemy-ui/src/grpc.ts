import { Table, tableFromIPC } from '@apache-arrow/ts';
import { GrpcWebFetchTransport } from '@protobuf-ts/grpcweb-transport';
import { QueryEngineClient } from './generated/query_engine.client';
import { RpcOptions, UnaryCall } from '@protobuf-ts/runtime-rpc';
import { AUTH_TOKEN_KEY } from './constants';
import { QueryStatus, QueryStatusResponse } from './generated/query_engine';

export const grpcClient = () => {
  const options: RpcOptions = {
    interceptors: [
      {
        // adds auth header to unary requests
        interceptUnary(next, method, input, options: RpcOptions): UnaryCall {
          if (!options.meta) {
            options.meta = {};
          }
          options.meta['Authorization'] =
            `Bearer ${localStorage.getItem(AUTH_TOKEN_KEY) || ''}`;
          options.meta['grpc-accept-encoding'] = 'identity';
          options.meta['grpc-encoding'] = 'identity';
          return next(method, input, options);
        },
      },
    ],
  };
  const transport = new GrpcWebFetchTransport({
    baseUrl: import.meta.env.VITE_PTOLEMY_API,
    ...options,
  });
  return new QueryEngineClient(transport);
};

export const runQuery = async (client: QueryEngineClient, query: string) => {
  const {
    response: { queryId, success, error },
  } = await client.query({ query });
  if (success) {
    return queryId;
  } else {
    throw error;
  }
};

const FINAL_STATES = [
  QueryStatus.CANCELLED,
  QueryStatus.COMPLETED,
  QueryStatus.FAILED,
];

export const waitForQuery = async (
  client: QueryEngineClient,
  queryId: string,
): Promise<QueryStatusResponse> => {
  let { response } = await client.getQueryStatus({ queryId });
  while (!FINAL_STATES.includes(response.status)) {
    response = (await client.getQueryStatus({ queryId })).response;
  }
  return response;
};

export const fetchData = async (
  client: QueryEngineClient,
  queryId: string,
): Promise<[string[], string[][]]> => {
  const batches: Uint8Array[] = [];

  try {
    console.log(`Fetching data for query ID: ${queryId}`);

    // Get the metadata first to check if there are any batches
    const statusResponse = await client.getQueryStatus({ queryId });
    console.log(`Query status:`, statusResponse.response);

    // Get metadata without using find
    let totalBatches = 0;
    let columnNames: string[] = [];

    // Check if metadata exists and how to access it
    if (statusResponse.response.metadata) {
      console.log(
        'Metadata object exists, dumping structure:',
        statusResponse.response.metadata,
      );

      // Try to access metadata properties based on the structure
      try {
        if (typeof statusResponse.response.metadata === 'object') {
          // If it's a direct property access
          totalBatches = statusResponse.response.metadata.totalBatches || 0;
          columnNames = statusResponse.response.metadata.columnNames;
        }
      } catch (e) {
        console.error('Error accessing metadata properties:', e);
      }
    }

    console.log(
      `Processed metadata: totalBatches=${totalBatches}, columnNames=${columnNames.join(',')}`,
    );

    if (totalBatches === 0) {
      console.warn('No batches to fetch according to metadata');
      // Return empty CSV with column names if available
      if (columnNames.length > 0) {
        return [columnNames, []];
      }
      return [[], []];
    }

    // Proceed with fetching batches
    console.log('Initiating batch fetch...');
    const call = client.fetchBatch({ queryId });

    // Add a timeout just in case
    const timeoutPromise = new Promise<void>((_, reject) => {
      setTimeout(
        () => reject(new Error('Timeout while fetching batches')),
        30000,
      );
    });

    // Use Promise.race to add timeout to the batch fetching process
    await Promise.race([
      (async () => {
        try {
          for await (const message of call.responses) {
            console.log(
              `Received batch, size: ${message.data?.byteLength || 0} bytes, isLastBatch: ${message.isLastBatch}`,
            );

            // Make sure data exists and is not empty
            if (message.data && message.data.byteLength > 0) {
              batches.push(message.data);
            } else {
              console.warn('Received empty batch data');
            }

            if (message.isLastBatch) {
              console.log('Received last batch marker');
              break;
            }
          }
        } catch (iterationError) {
          console.error('Error during batch iteration:', iterationError);
          throw iterationError;
        }
      })(),
      timeoutPromise,
    ]);

    await call.status;
    await call.trailers;

    console.log(`Received ${batches.length} batches`);

    if (!batches.length) {
      console.warn('Did not receive any data batches from API');
      // Return empty CSV with headers if we have column names
      if (columnNames.length > 0) {
        return [columnNames, []];
      }
      throw new Error('Did not get any data batches from API!');
    }

    return processData(batches);
  } catch (error) {
    console.error('Error in fetchData:', error);

    // If we have metadata but no batches, this might be an empty result set
    if (batches.length === 0) {
      try {
        const statusResponse = await client.getQueryStatus({ queryId });
        console.log(`Final query status:`, statusResponse.response);

        // Try to get column names for empty result
        let columnNames: string[] = [];
        try {
          if (
            statusResponse.response.metadata &&
            typeof statusResponse.response.metadata === 'object' &&
            'metadata:column_names' in statusResponse.response.metadata
          ) {
            const namesStr =
              statusResponse.response.metadata['metadata:column_names'];
            columnNames =
              typeof namesStr === 'string' ? JSON.parse(namesStr) : [];
          }
        } catch (e) {
          console.error('Error getting column names:', e);
        }

        // If we have column names, return empty CSV with headers
        if (columnNames.length > 0) {
          return [columnNames, []];
        }
      } catch (metadataError) {
        console.error(
          'Error fetching metadata after batch failure:',
          metadataError,
        );
      }
    }

    throw error;
  }
};

// Enhanced processData function with more debugging and safety
export const processData = (data: Uint8Array[]): [string[], string[][]] => {
  try {
    console.log(`Processing ${data.length} data batches`);

    if (data.length === 0) {
      return [[], []];
    }

    const table: Table = data.reduce(
      (prev: Table | null, curr: Uint8Array, index) => {
        if (curr.byteLength === 0) {
          console.warn(`Empty batch at index ${index}, skipping`);
          return prev;
        }

        try {
          if (prev === null) {
            console.log(`Creating first table from batch ${index}`);
            try {
              return tableFromIPC(curr);
            } catch (e) {
              let message;
              if (e instanceof Error) message = e.message
              else message = String(e);
              console.warn(`Error with default tableFromIPC: ${message}`);
              throw e;
            }
          } else {
            console.log(`Concatenating batch ${index}`);
            try {
              return prev.concat(tableFromIPC(curr));
            } catch (e) {
              let message;
              if (e instanceof Error) message = e.message
              else message = String(e);
              console.warn(`Error concatenating batch ${index}: ${message}`);
              throw e;
            }
          }
        } catch (e) {
          console.error(`Failed to process batch ${index}:`, e);
          throw e;
        }
      },
      null,
    ) as Table;

    if (table === null) {
      console.error('No valid table created from batches');
      throw new Error('No data received or all batches invalid!');
    }

    console.log(
      `Successfully created table with schema: ${JSON.stringify(table.schema.fields.map((f) => f.name))}`,
    );
    console.log(
      `Table has ${table.numRows} rows and ${table.schema.fields.length} columns`,
    );

    // Use Arrow's built-in toArray method to get the data as an array of objects
    const records = table.toArray();

    // Convert the array of objects to CSV
    const columns = table.schema.fields.map((f) => f.name);

    const rows = [];
    for (const record of records) {
      // Ensure all values are properly stringified
      const row = columns.map((col) => {
        const value = record[col];
        if (value === null || value === undefined) {
          return '';
        }
        // Ensure booleans are properly stringified
        if (typeof value === 'boolean') {
          return value.toString();
        }
        return String(value);
      });

      rows.push(row);
    }

    return [columns, rows];
  } catch (error) {
    console.error('Error in processData:', error);
    throw error;
  }
};

export async function runQueryAndGetData(
  query: string,
): Promise<[string[], string[][]]> {
  const client = grpcClient();

  try {
    console.log(
      `Submitting query: ${query.substring(0, 100)}${query.length > 100 ? '...' : ''}`,
    );

    const queryId = await runQuery(client, query);
    console.log(`Query submitted successfully, received queryId: ${queryId}`);

    // Wait for query to complete
    console.log(`Waiting for query ${queryId} to complete...`);
    const queryStatus = await waitForQuery(client, queryId);
    console.log(
      `Query completed with status: ${QueryStatus[queryStatus.status]}`,
    );

    if (queryStatus.error) {
      console.error(`Query error: ${queryStatus.error}`);
    }

    // Log metadata if available
    console.log('Query metadata:', queryStatus);

    switch (queryStatus.status) {
      case QueryStatus.COMPLETED: {
        // Check if there are any rows in the result based on metadata
        // Get metadata without using find (since metadata might not be an array)
        let totalRows = 0;
        let columnNames = [];

        try {
          // First, check if metadata is a Map or Record-like object
          if (
            queryStatus.metadata &&
            typeof queryStatus.metadata === 'object'
          ) {
            console.log('Metadata is an object, accessing properties directly');

            // Try accessing using dot notation first
            if ('metadata:total_rows' in queryStatus.metadata) {
              totalRows =
                parseInt(queryStatus.metadata['metadata:total_rows'] as string, 10) || 0;
            }

            // Try accessing column names
            if ('metadata:column_names' in queryStatus.metadata) {
              try {
                const namesStr = queryStatus.metadata['metadata:column_names'];
                columnNames =
                  typeof namesStr === 'string' ? JSON.parse(namesStr) : [];
              } catch (e) {
                console.error('Error parsing column names:', e);
              }
            }
          } else {
            console.log(
              'Metadata is not accessible as an object, logging full structure',
            );
            console.log(JSON.stringify(queryStatus));
          }
        } catch (metadataError) {
          console.error('Error accessing metadata:', metadataError);
        }

        console.log(
          `Processed metadata: totalRows=${totalRows}, columnNames=${columnNames.join(',')}`,
        );

        if (totalRows === 0 && columnNames.length > 0) {
          console.log('Query completed successfully but returned no rows');
          // Return empty CSV with headers
          return [columnNames, []];
        }

        console.log(`Fetching data for query ${queryId}`);
        try {
          const [columns, data] = await fetchData(client, queryId);
          console.log(
            `Successfully fetched and processed data (${data.length} bytes)`,
          );
          return [columns, data];
        } catch (fetchError) {
          let message;
          if (fetchError instanceof Error) message = fetchError.message;
          else message = String(fetchError);
          console.error(`Error fetching data: ${message}`);

          // If we failed to fetch data but have column names, return empty CSV
          if (columnNames.length > 0) {
            return [columnNames, []];
          }

          throw fetchError;
        }
      }
      case QueryStatus.CANCELLED:
        throw new Error('Query was canceled');

      case QueryStatus.FAILED:
        throw new Error(queryStatus.error || 'Query failed');

      default:
        throw new Error(`Unknown query status: ${queryStatus.status}`);
    }
  } catch (error) {
    console.error('Error in runQueryAndGetData:', error);
    throw error;
  }
}
