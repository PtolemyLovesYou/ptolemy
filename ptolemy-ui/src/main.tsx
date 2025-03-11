import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import Cookies from 'js-cookie';
import './index.css';
import App from './App.tsx';

import {
  ApolloProvider,
  ApolloClient,
  InMemoryCache,
  createHttpLink,
} from '@apollo/client';
import { setContext } from '@apollo/client/link/context';
import { AUTH_TOKEN_KEY } from './constants.ts';
import AuthProvider from '@/auth/provider.tsx';
import { ThemeProvider } from '@/components/theme/theme-provider.tsx';

const httpLink = createHttpLink({
  uri: `${import.meta.env.VITE_PTOLEMY_API}/graphql`,
});

const authLink = setContext((_, { headers }) => {
  const token = Cookies.get(AUTH_TOKEN_KEY);
  console.log(token, 'token');
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : '',
    },
  };
});

const client = new ApolloClient({
  link: authLink.concat(httpLink),
  cache: new InMemoryCache(),
  credentials: 'include',
});

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <AuthProvider>
      <ApolloProvider client={client}>
        <BrowserRouter>
          <ThemeProvider>
            <App />
          </ThemeProvider>
        </BrowserRouter>
      </ApolloProvider>
    </AuthProvider>
  </StrictMode>,
);
