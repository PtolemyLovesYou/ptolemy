"""Test log."""

import time
import requests
from tqdm.auto import tqdm
from ptolemy import Ptolemy

# get superadmin credentials
superadmin_jwt = requests.post(
    "http://localhost:8000/auth",
    json={
        "username": "admin",
        "password": "admin",
    },
    timeout=30,
).json()['token']

# dummy admin user
username = f"user_{int(time.time())}"
password = username

MUTATION = """
mutation CreateUserApiKey($name: String!, $durationDays: Int) {
  user {
    createUserApiKey(name: $name, durationDays: $durationDays) {
      apiKey {
        apiKey
      }
      ...Result
    }
  }
}

mutation CreateUser($username: String!, $password: String!, $isAdmin: Boolean!, $displayName: String) {
  user {
    create(
      userData: {username: $username, password: $password, isSysadmin: false, isAdmin: $isAdmin, displayName: $displayName}
    ) {
      ...Result
      user {
        ...ReturnsUser
      }
    }
  }
}

mutation CreateWorkspace($name: String!, $description: String, $adminUserId: Uuid!) {
  workspace {
    create(
      workspaceData: {name: $name, description: $description}
      adminUserId: $adminUserId
    ) {
      ...Result
      workspace {
        ...ReturnsWorkspace
      }
    }
  }
}

mutation CreateServiceApiKey($name: String!, $permission: ApiKeyPermissionEnum!, $workspaceId: Uuid!, $durationDays: Int) {
  workspace {
    createServiceApiKey(
      name: $name
      permission: $permission
      workspaceId: $workspaceId
      durationDays: $durationDays
    ) {
      ...Result
      apiKey {
        apiKey
      }
    }
  }
}

fragment ReturnsWorkspace on Workspace {
  id
  name
  description
  archived
  createdAt
  updatedAt
}

fragment ReturnsUser on User {
  displayName
  id
  isAdmin
  isSysadmin
  status
  username
}

fragment Result on GQLResult {
  error {
    field
    message
  }
  success
}
"""

admin_user = requests.post(
    "http://localhost:8000/graphql",
    json={
        "query": MUTATION,
        "operationName": "CreateUser",
        "variables": {
            "username": username,
            "password": password,
            "isAdmin": True,
            "displayName": "Test User",
        },
    },
    headers={
        "Authorization": f"Bearer {superadmin_jwt}",
        "Content-Type": "application/json"
    },
    timeout=30
).json()['data']['user']['create']['user']

print(admin_user)

# login as admin user and create workspace
admin_jwt = requests.post(
    "http://localhost:8000/auth",
    json={
        "username": username,
        "password": password,
    },
    timeout=30,
).json()['token']

workspace = requests.post(
    "http://localhost:8000/graphql",
    json={
        "query": MUTATION,
        "operationName": "CreateWorkspace",
        "variables": {
            "name": f"test_workspace_{int(time.time())}",
            "adminUserId": admin_user['id'],
        },
    },
    headers={
        "Authorization": f"Bearer {admin_jwt}",
        "Content-Type": "application/json"
    },
    timeout=30
).json()['data']['workspace']['create']['workspace']

wk_name = workspace['name']
wk_id = workspace['id']

print(wk_name, wk_id)

# create personal api key
personal_api_key = requests.post(
    "http://localhost:8000/graphql",
    json={
        "query": MUTATION,
        "operationName": "CreateUserApiKey",
        "variables": {
            "name": "test_api_key",
            "durationDays": 1,
        },
    },
    headers={
        "Authorization": f"Bearer {admin_jwt}",
        "Content-Type": "application/json"
    },
    timeout=30
).json()['data']['user']['createUserApiKey']['apiKey']['apiKey']

print("PERSONAL API KEY: ", personal_api_key)

# create service api key
service_api_key = requests.post(
    "http://localhost:8000/graphql",
    json={
        "query": MUTATION,
        "operationName": "CreateServiceApiKey",
        "variables": {
            "name": "test_api_key",
            "permission": "READ_WRITE",
            "workspaceId": wk_id,
            "durationDays": 1,
        },
    },
    headers={
        "Authorization": f"Bearer {admin_jwt}",
        "Content-Type": "application/json"
    },
    timeout=30
).json()['data']['workspace']['createServiceApiKey']['apiKey']['apiKey']

print("SERVICE API KEY: ", service_api_key)

# create ptolemy client
client = Ptolemy(
    base_url="http://localhost:8000",
    api_key=service_api_key,
    workspace_name=wk_name,
    autoflush=False,
    batch_size=1024
)

N = 100

start = time.time()
for _ in tqdm(list(range(N))):
    sys = client.trace("test_trace", version='1.2.3', environment='dev')
    with sys:
        sys.inputs(
            foo={"bar": "baz"},
            baz=1,
            qux=True,
            test_str="this is a string",
            test_float=0.93
            )
        subsys = sys.child("sub_trace", version='1.2.3', environment='dev')
        with subsys:
            comp = subsys.child("comp_trace", version='1.2.3', environment='dev')
            with comp:
                subcomp = comp.child("subcomp_trace", version='1.2.3', environment='dev')
                with subcomp:
                    pass

end = time.time()
client.flush()
print(((end - start) / N) * 1000)

for i in client.sql("select 1"):
    print(i)
