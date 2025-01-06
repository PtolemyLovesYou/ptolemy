"""Login."""

from urllib.parse import urljoin
import requests
import questionary
from prompt_toolkit import PromptSession, print_formatted_text as printf
from ..models.auth import User, Workspace
from ..models.gql import GQLQuery
from ..gql import GET_USER_WORKSPACES


def login(session: PromptSession):
    """Login."""
    printf("Welcome to Ptolemy CLI!")
    username = session.prompt("Please enter your username:\n> ", is_password=False)
    password = session.prompt("Please enter your password:\n> ", is_password=True)

    resp = requests.post(
        urljoin("http://localhost:8000", "/auth"),
        json={"username": username, "password": password},
        timeout=5,
    )

    if resp.ok:
        data = resp.json()
        return User(
            id=data["id"],
            username=data["username"],
            is_admin=data["is_admin"],
            is_sysadmin=data["is_sysadmin"],
            display_name=data.get("display_name"),
            status=data["status"].upper(),
        )

    raise ValueError("Failed to login.")


def select_workspace(usr: User) -> Workspace:
    """Select workspaces."""
    resp = GQLQuery.query(GET_USER_WORKSPACES, {"Id": usr.id.hex})
    workspaces = {w.name: w.to_model() for w in resp.users()[0].workspaces}

    wk = questionary.select(
        "Select a workspace:",
        choices=workspaces,
        use_shortcuts=True,
    ).ask()

    return workspaces[wk]
