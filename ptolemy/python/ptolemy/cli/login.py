"""Login."""

from urllib.parse import urljoin
import requests
import typer
from prompt_toolkit import PromptSession
from ..models.auth import User


def login(session: PromptSession):
    """Login."""
    typer.echo("Welcome to Ptolemy CLI!")
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
