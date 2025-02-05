import { useQuery } from "urql";
import { graphql } from "../gql";
import { useState } from "react";

export function ChatList() {
  const [chatsRes] = useQuery({ query: queryList });
  const [chatId, setChatId] = useState<string | null>(null);

  return (
    <div className="flex gap-4">
      <div className="flex flex-col gap-4">
        {chatsRes.data?.chats.map((chat) => (
          <div
            key={chat.id}
            className="px-2 py-1 bg-white cursor-pointer data-[selected=true]:bg-blue-500 data-[selected=true]:text-white rounded"
            onClick={() => setChatId(chat.id)}
            data-selected={chatId === chat.id}
          >
            {chat.createdAt}
          </div>
        ))}
      </div>
      {chatId && <Chat chatId={chatId} />}
    </div>
  );
}

function Chat({ chatId }: { chatId: string }) {
  const [chatRes] = useQuery({ query: queryChat, variables: { id: chatId } });

  return (
    <div className="flex flex-col gap-4">
      <div className="grow flex flex-col gap-4">
        {chatRes.data?.chat?.messages.map((message: any) => (
          <div
            key={message.createdAt}
            className={`px-2 py-1 bg-white rounded ${
              message.role === "user" ? "self-end" : "self-start"
            }`}
          >
            {message.content}
          </div>
        ))}
      </div>
      {/* <div className="flex flex-col gap-4">
        <textarea className="w-96 h-40 border rounded-sm p-2" />
        <button>Send</button>
      </div> */}
    </div>
  );
}

const queryList = graphql(`
  query chats {
    chats(includeDeleted: false) {
      id
      createdAt
    }
  }
`);

const queryChat = graphql(`
  query chat($id: ID!) {
    chat(id: $id) {
      id
      createdAt
      messages {
        role
        content
        createdAt
      }
    }
  }
`);
