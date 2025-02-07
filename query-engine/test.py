"""Serialization example."""

from io import BytesIO
import pandas as pd
import redis

redis_conn = redis.Redis(host="localhost", port=6379, db=0)

msg = {
    "action": "start",
    "query_id": "1",
    "schema_name": "test",
    "role_name": "test",
    "query": "select 1",
}

redis_conn.xadd("ptolemy:query", msg)

print(redis_conn.hget("ptolemy:query:1", "status"))

c = 0

while True:
    result = redis_conn.hget("ptolemy:query:1", f"result:{c}")
    if result is None:
        break
    # deserialize from bytes
    buf = BytesIO(result)
    df = pd.read_feather(buf)
    print(df)
    c += 1

# conn = duckdb.connect()

# df = pd.DataFrame([{"a": x, "b": x**2} for x in range(1000)])

# result = map(
#     lambda batch: batch.serialize().to_pybytes(),
#     conn.sql("select * from df").fetch_arrow_reader(batch_size=100),
# )

# for r in result:
#     print(type(r))

# At user creation
# 1. Create schema for credentials provided
# 2. Create role that only allows read-only access to that schema only

# Query Preparation
# 1. Fetch schema + role for data
# 2. Create session id
# 3. Send data to query executor

# Query Execution
# 1. Connect to database with schema to be used
# 2. Set role
# 3. Run query
# 4. Disconnect
# 5. Send results to Redis
# 6. Send query_id to API

# Query results
# 1. Send query_id to client
# 2. For each batch, pull from Redis and send to client

# Objects
# Query: Has one result set. ID: query_id
