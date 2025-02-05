import { useEffect, useState } from "react";
import { Provider } from "urql";
import { client } from "../urql";
import { ChatList } from "./ChatList";
import { Notes } from "./Notes";
import { PostButton } from "./PostButton";
import { Button } from "./Button";

function App() {
  const [state, setState] = useState(parseHash(location.hash));
  const [mode, setMode] = useState<"notes" | "chats">("notes");

  useEffect(() => {
    const onHashChange = () => {
      setState(parseHash(location.hash));
    };
    window.addEventListener("hashchange", onHashChange);
    return () => {
      window.removeEventListener("hashchange", onHashChange);
    };
  });

  return (
    <Provider value={client}>
      <div className="w-dvw h-dvh flex flex-col items-stretch bg-gray-200 text-gray-800">
        <header className="p-4 flex">
          <h1 className="mr-auto text-4xl font-bold">anlg</h1>
          <div className="flex gap-2">
            <Button
              onClick={() => setMode("notes")}
              selected={mode === "notes"}
            >
              notes
            </Button>
            <Button
              onClick={() => setMode("chats")}
              selected={mode === "chats"}
            >
              chats
            </Button>
            <PostButton />
          </div>
        </header>
        <div className="p-4 flex flex-col gap-4">
          <div className="flex-1">
            {mode === "notes" && <Notes noteId={state.noteId} />}
            {mode === "chats" && <ChatList />}
          </div>
        </div>
      </div>
    </Provider>
  );
}

export default App;

function parseHash(hash: string) {
  const matchWithNotes = hash.match(/#\/notes\/([^/]+)/);
  if (matchWithNotes) {
    const [, id] = matchWithNotes;
    return {
      noteId: id,
    };
  }
  return {
    noteId: null,
  };
}
