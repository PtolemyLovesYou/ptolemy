import { AUTH_TOKEN_KEY } from "@/constants";

export const authenticate = async (username: string, password: string) => {
    const response = await fetch(`${import.meta.env.VITE_PTOLEMY_API}/auth`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            username,
            password
        })
    });
    const { token } = await response.json();
    if (token) {
        localStorage.setItem(AUTH_TOKEN_KEY, token)
        return token
    }
}
