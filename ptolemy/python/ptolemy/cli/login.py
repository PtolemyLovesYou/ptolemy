"""Login."""

from urllib.parse import urljoin
import requests
from prompt_toolkit import PromptSession, print_formatted_text as printf
from ..models.auth import User


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
