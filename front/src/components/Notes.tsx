import { useCallback, useState } from "react";
import { useMutation, useQuery } from "urql";
import { graphql } from "../gql";

import { setNote } from "../hashChanger";
import { Dialog } from "./Dialog";

const styleTime = " text-sm opacity-80";

export function Notes({ noteId }: { noteId: string | null }) {
  const [notesRes] = useQuery({ query: notesQuery });

  return (
    <div className="flex gap-4 flex-wrap items-start">
      {notesRes.error && "The server is unavailable."}
      {notesRes.data?.notes.map(
        (note: {
          id: string;
          content: string;
          createdAt: string;
          updatedAt: string;
          deletedAt?: string | null;
        }) =>
          note.deletedAt ? null : (
            <div
              className="min-w-[10rem] min-h-10 max-w-[20rem] px-2 py-4 bg-white rounded-lg cursor-pointer"
              key={note.id}
              onClick={(e) => {
                setNote(note.id);
                e.stopPropagation();
              }}
            >
              <div className="whitespace-pre-wrap break-words">{note.content}</div>
              <div className={styleTime}>
                {relativeTimeFormat(new Date(note.updatedAt))}
              </div>
            </div>
          )
      )}
      {noteId && (
        <Dialog onClose={() => setNote(null)}>
          <Note noteId={noteId} />
        </Dialog>
      )}
    </div>
  );
}

const notesQuery = graphql(`
  query notes {
    notes(includeDeleted: false) {
      id
      content
      createdAt
      updatedAt
      deletedAt
    }
  }
`);

const deleteMutation = graphql(`
  mutation delete($id: ID!) {
    deleteNote(noteId: $id) {
      id
    }
  }
`);

function Note({
  noteId,
}: {
  noteId: string;
}) {
  const [noteRes] = useQuery({ query: noteQuery, variables: { id: noteId } });
  const note = noteRes.data?.note ?? { id: "" };

  const [isProcessing, setIsProcessing] = useState(false);
  const [, deleteMut] = useMutation(deleteMutation);
  const [, requestCompanionsCommentMut] = useMutation(
    requestCompanionsCommentMutation
  );
  const [, addCommentMut] = useMutation(addCommentMutation);

  const deleteNote = async () => {
    setIsProcessing(true);
    await deleteMut({ id: note.id });
    setIsProcessing(false);
  };

  const requestCompanionsComment = useCallback(async () => {
    setIsProcessing(true);
    await requestCompanionsCommentMut({ noteId: note.id });
    setIsProcessing(false);
  }, [note.id, requestCompanionsCommentMut]);

  const [text, setText] = useState("");
  const addComment = async () => {
    setIsProcessing(true);
    await addCommentMut({ noteId: note.id, content: text });
    setText("");
    setIsProcessing(false);
  };

  if (!('content' in note))
    return (
      <div className="max-w-[80vw]">
        <pre>loading...</pre>
      </div>
    );

  return (
    <div className="max-w-[80vw]">
      <pre className="whitespace-pre-wrap break-words">{note.content}</pre>
      <div className="flex gap-2">
        <span className={styleTime}>
          {formatDate(new Date(note.createdAt))}
        </span>
        <button onClick={deleteNote} disabled={isProcessing}>Delete</button>
      </div>
      <div className="border-t my-1"></div>
      {note.messages.map((m) => (
        <div className="my-2" key={m.createdAt}>
          <header className="flex items-center gap-2">
            <span style={{ color: m.role === "USER" ? "#666" : "#55f" }}>
              {m.role === "USER" ? "you" : "bot"}
            </span>
            <span className={styleTime}>
              {relativeTimeFormat(new Date(m.createdAt))}
            </span>
          </header>
          <pre className="whitespace-pre-wrap break-words">{m.content}</pre>
        </div>
      ))}
      <div className="flex gap-2">
        <textarea className="flex-1 border rounded p-2" value={text} onChange={(e) => setText(e.target.value)} />
        <div className="flex flex-col gap-2">
          <button onClick={addComment} disabled={isProcessing}>Post</button>
          <button onClick={requestCompanionsComment} disabled={isProcessing}>Request</button>
        </div>
      </div>
    </div>
  );
}

const noteQuery = graphql(`
  query note($id: ID!) {
    note(id: $id) {
      id
      content
      messages {
        role
        content
        createdAt
      }
      createdAt
      updatedAt
      deletedAt
    }
  }
`);

const requestCompanionsCommentMutation = graphql(`
  mutation requestCompanionsComment($noteId: ID!) {
    requestCompanionsComment(noteId: $noteId) {
      id
    }
  }
`);

const addCommentMutation = graphql(`
  mutation addComment($noteId: ID!, $content: String!) {
    addComment(noteId: $noteId, content: $content) {
      id
    }
  }
`);

function formatDate(date: Date) {
  return date.toISOString().replace(/T/, " ").replace(/\..+/, "");
}

function relativeTimeFormat(data: Date) {
  const diff = Date.now() - data.getTime();
  const seconds = Math.floor(diff / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);
  const weeks = Math.floor(days / 7);
  const months = Math.floor(days / 30);
  const years = Math.floor(days / 365); // not accurate (>_<;)

  if (seconds < 60) return "just now";
  if (minutes < 60) return `${minutes} minutes ago`;
  if (hours < 24) return `${hours} hours ago`;
  if (days < 7) return `${days} days ago`;
  if (weeks < 4) return `${weeks} weeks ago`;
  if (months < 12) return `${months} months ago`;
  return `${years} years ago`;
}
