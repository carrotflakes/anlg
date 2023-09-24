import { Client, cacheExchange, fetchExchange } from 'urql';
import { apiUrl } from './config';
import { AuthConfig, AuthUtilities, authExchange } from '@urql/exchange-auth';

export const client = new Client({
  url: new URL('/graphql', apiUrl).toString(),
  exchanges: [cacheExchange,
    authExchange(async (utils: AuthUtilities) => {
      const token = localStorage.getItem('anlg-token');

      return {
        addAuthToOperation(operation) {
          if (!token) return operation;
          return utils.appendHeaders(operation, {
            Authorization: `Bearer ${token}`,
          });
        },
      } as AuthConfig;
    }),
    fetchExchange],
});
