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
    "\n  query chats {\n    chats(includeDeleted: false) {\n      id\n      createdAt\n      messages {\n        role\n        content\n        createdAt\n      }\n    }\n  }\n": types.ChatsDocument,
    "\n  query notes {\n    notes(includeDeleted: false) {\n      id\n      content\n      createdAt\n      updatedAt\n      deletedAt\n    }\n  }\n": types.NotesDocument,
    "\n  mutation delete($id: ID!) {\n    deleteNote(noteId: $id) {\n      id\n    }\n  }\n": types.DeleteDocument,
    "\n  query note($id: ID!) {\n    note(id: $id) {\n      id\n      content\n      messages {\n        role\n        content\n        createdAt\n      }\n      createdAt\n      updatedAt\n      deletedAt\n    }\n  }\n": types.NoteDocument,
    "\n  mutation requestCompanionsComment($noteId: ID!) {\n    requestCompanionsComment(noteId: $noteId) {\n      id\n    }\n  }\n": types.RequestCompanionsCommentDocument,
    "\n  mutation addComment($noteId: ID!, $content: String!) {\n    addComment(noteId: $noteId, content: $content) {\n      id\n    }\n  }\n": types.AddCommentDocument,
    "\n  mutation post($content: String!) {\n    post(content: $content) {\n      id\n    }\n  }\n": types.PostDocument,
    "\n  mutation newChat($content: String!) {\n    newChat(content: $content) {\n      id\n    }\n  }\n": types.NewChatDocument,
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
export function graphql(source: "\n  query chats {\n    chats(includeDeleted: false) {\n      id\n      createdAt\n      messages {\n        role\n        content\n        createdAt\n      }\n    }\n  }\n"): (typeof documents)["\n  query chats {\n    chats(includeDeleted: false) {\n      id\n      createdAt\n      messages {\n        role\n        content\n        createdAt\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query notes {\n    notes(includeDeleted: false) {\n      id\n      content\n      createdAt\n      updatedAt\n      deletedAt\n    }\n  }\n"): (typeof documents)["\n  query notes {\n    notes(includeDeleted: false) {\n      id\n      content\n      createdAt\n      updatedAt\n      deletedAt\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation delete($id: ID!) {\n    deleteNote(noteId: $id) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation delete($id: ID!) {\n    deleteNote(noteId: $id) {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query note($id: ID!) {\n    note(id: $id) {\n      id\n      content\n      messages {\n        role\n        content\n        createdAt\n      }\n      createdAt\n      updatedAt\n      deletedAt\n    }\n  }\n"): (typeof documents)["\n  query note($id: ID!) {\n    note(id: $id) {\n      id\n      content\n      messages {\n        role\n        content\n        createdAt\n      }\n      createdAt\n      updatedAt\n      deletedAt\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation requestCompanionsComment($noteId: ID!) {\n    requestCompanionsComment(noteId: $noteId) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation requestCompanionsComment($noteId: ID!) {\n    requestCompanionsComment(noteId: $noteId) {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation addComment($noteId: ID!, $content: String!) {\n    addComment(noteId: $noteId, content: $content) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation addComment($noteId: ID!, $content: String!) {\n    addComment(noteId: $noteId, content: $content) {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation post($content: String!) {\n    post(content: $content) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation post($content: String!) {\n    post(content: $content) {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation newChat($content: String!) {\n    newChat(content: $content) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation newChat($content: String!) {\n    newChat(content: $content) {\n      id\n    }\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;