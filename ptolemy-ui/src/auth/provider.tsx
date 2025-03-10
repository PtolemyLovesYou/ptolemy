import React, { createContext, useState, useContext } from 'react';
import Cookies from 'js-cookie';

import { AUTH_TOKEN_KEY } from '@/constants';

interface ProviderProps {
  user: string | null;
  token: string;
  login(username: string, password: string): void;
  logout(): void;
}

const AuthContext = createContext<ProviderProps>({
  user: null,
  token: '',
  login: () => {},
  logout: () => {},
});

const AuthProvider = ({ children }: { children: React.ReactNode }) => {
  const storedInfo = localStorage.getItem('user')
    ? JSON.parse(localStorage.getItem('user') || '{}')
    : null;
  const [user, setUser] = useState<string | null>(storedInfo?.email);
  const [token, setToken] = useState(
    storedInfo?.token || Cookies.get(AUTH_TOKEN_KEY) || '',
  );

  const login = async (username: string, password: string) => {
    const response = await fetch(`${import.meta.env.VITE_PTOLEMY_API}/auth`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username,
        password,
      }),
    });
    const { token, user } = await response.json();
    if (user) {
      setUser(user);
    }
    if (token) {
      setToken(token);
      Cookies.set(AUTH_TOKEN_KEY, token, { expires: 7 });
      window.location.reload();
    } // TODO - handle loginFailure; handle detecting expired token
  };

  const logout = () => {
    setUser(null);
    setToken('');
    Cookies.remove(AUTH_TOKEN_KEY);
    localStorage.removeItem('user');
  };

  return (
    <AuthContext.Provider value={{ user, token, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export default AuthProvider;

export const useAuth = () => {
  return useContext(AuthContext);
};
