import { Client, cacheExchange, fetchExchange } from 'urql';

export const client = new Client({
  url: 'http://localhost:8000/graphql',
  exchanges: [cacheExchange, fetchExchange],
});
