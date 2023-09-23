import { Client, cacheExchange, fetchExchange } from 'urql';
import { apiUrl } from './config';

export const client = new Client({
  url: new URL('/graphql', apiUrl).toString(),
  exchanges: [cacheExchange, fetchExchange],
});
