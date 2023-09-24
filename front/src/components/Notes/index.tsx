import { useState } from "react";
import { useMutation, useQuery } from "urql";
import { graphql } from "../../gql";

import { Dialog } from "../Dialog";
import styles from "./index.module.scss";

export function Notes() {
  const [res] = useQuery({ query: aQuery });
  const [notesRes] = useQuery({ query: notesQuery });
  const [selected, setSelected] = useState<string | null>(null);

  // const [, deleteMut] = useMutation(deleteMutation);
  // const deleteNote = async (id: string) => {
  //   await deleteMut({ id });
  //   refresh({ requestPolicy: "network-only" });
  // };

  const selectedNote = notesRes.data?.notes.find((n) => n.id === selected);

  return (
    <div>
      {res.data?.add === 3 || "The server is unavailable."}
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
              className={styles.card}
              key={note.id}
              onClick={(e) => {
                setSelected(note.id);
                e.stopPropagation();
              }}
            >
              <pre>{note.content}</pre>
              <div className={styles.time}>
                {relativeTimeFormat(new Date(note.updatedAt))}
              </div>
            </div>
          )
      )}
      {selected && (
        <Dialog onClose={() => setSelected(null)}>
          {selectedNote && <Note note={selectedNote} />}
        </Dialog>
      )}
    </div>
  );
}

const aQuery = graphql(`
  query a {
    add(a: 1, b: 2)
  }
`);

const notesQuery = graphql(`
  query notes {
    notes {
      id
      content
      createdAt
      updatedAt
      deletedAt
    }
  }
`);

const deleteMutation = graphql(`
  mutation delete($id: String!) {
    deleteNote(noteId: $id) {
      id
    }
  }
`);

function Note({
  note,
}: {
  note: {
    id: string;
    content: string;
    createdAt: string;
    updatedAt: string;
    deletedAt?: string | null;
  };
}) {
  const [, deleteMut] = useMutation(deleteMutation);
  const deleteNote = async () => {
    await deleteMut({ id: note.id });
  };

  return (
    <div>
      <pre>{note.content}</pre>
      {formatDate(new Date(note.createdAt))}
      <button onClick={deleteNote}>Delete</button>
    </div>
  );
}

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
