"""Test graphql"""
from ptolemy._core import GraphQLClient

client = GraphQLClient("http://localhost:8000/graphql")

user = client.get_user_by_name("asdf")

print(user.id, user.username, user.is_admin, user.is_sysadmin)

for i in client.get_user_workspaces(str(user.id)):
    print(i)

for i in client.get_user_api_keys(str(user.id)):
    print(i)
