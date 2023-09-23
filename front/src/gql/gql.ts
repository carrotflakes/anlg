/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    "\n  query a {\n    add(a: 1, b: 2)\n  }\n": types.ADocument,
    "\n  query notes {\n    notes {\n      id\n      content\n      createdAt\n    }\n  }\n": types.NotesDocument,
    "\n  mutation post($content: String!) {\n    post(content: $content)\n  }\n": types.PostDocument,
    "\n  mutation delete($id: String!) {\n    delete(noteId: $id)\n  }\n": types.DeleteDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query a {\n    add(a: 1, b: 2)\n  }\n"): (typeof documents)["\n  query a {\n    add(a: 1, b: 2)\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query notes {\n    notes {\n      id\n      content\n      createdAt\n    }\n  }\n"): (typeof documents)["\n  query notes {\n    notes {\n      id\n      content\n      createdAt\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation post($content: String!) {\n    post(content: $content)\n  }\n"): (typeof documents)["\n  mutation post($content: String!) {\n    post(content: $content)\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation delete($id: String!) {\n    delete(noteId: $id)\n  }\n"): (typeof documents)["\n  mutation delete($id: String!) {\n    delete(noteId: $id)\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;