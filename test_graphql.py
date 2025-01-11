"""Test graphql"""
from ptolemy._core import GraphQLClient

client = GraphQLClient("http://localhost:8000/graphql")

_, user = client.login("admin", "admin")

print(user)

for i in client.get_user_workspaces(str(user.id)):
    print(i)

for i in client.get_user_api_keys(str(user.id)):
    print(i)
