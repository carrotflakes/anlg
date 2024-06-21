import { useQuery } from "urql";
import { graphql } from "../gql";

export function ChatList() {
  const [chatsRes] = useQuery({ query });

  return (
    <div className="ChatList">
      <h1>Chat</h1>
      {chatsRes.data?.chats.map((chat) => (
        <div key={chat.id}>{chat.createdAt}</div>
      ))}
    </div>
  );
}

const query = graphql(`
  query chats {
    chats(includeDeleted: false) {
      id
      createdAt
    }
  }
`);
