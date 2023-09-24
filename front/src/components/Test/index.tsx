import { useMutation, useQuery } from "urql";
import { graphql } from "../../gql";
import React, { useEffect, useRef, useState } from "react";

import styles from "./index.module.scss";

export function Test() {
  const [res] = useQuery({ query: aQuery });
  const [notesRes, refresh] = useQuery({ query: notesQuery });
  const [, post] = useMutation(postMutation);
  const [text, setText] = useState("");
  const [selected, setSelected] = useState<string | null>(null);
  const [showPostForm, setShowPostForm] = useState(false);

  // const [, deleteMut] = useMutation(deleteMutation);
  // const deleteNote = async (id: string) => {
  //   await deleteMut({ id });
  //   refresh({ requestPolicy: "network-only" });
  // };

  const submit = async () => {
    await post({ content: text });
    refresh({ requestPolicy: "network-only" });
    setText("");
  };

  const selectedNote = notesRes.data?.notes.find((n) => n.id === selected);

  return (
    <div>
      {res.data?.add === 3 || "The server is unavailable."}
      {notesRes.data?.notes.map(
        (note: { id: string; content: string; createdAt: string }) => (
          <div
            className={styles.card}
            key={note.id}
            onClick={(e) => {
              setSelected(note.id);
              e.stopPropagation();
            }}
          >
            <pre>{note.content}</pre>
            {/* - {new Date(note.createdAt).toISOString()}{" "}
            - <button onClick={() => deleteNote(note.id)}>x</button> */}
          </div>
        )
      )}
      <button onClick={() => setShowPostForm(true)}>+</button>
      {selected && (
        <Dialog onClose={() => setSelected(null)}>
          {selectedNote && <Note note={selectedNote} />}
        </Dialog>
      )}
      {showPostForm && (
        <Dialog onClose={() => setShowPostForm(false)}>
          <textarea value={text} onChange={(e) => setText(e.target.value)} />
          <button onClick={submit}>Post</button>
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

const postMutation = graphql(`
  mutation post($content: String!) {
    post(content: $content) {
      id
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

function Dialog({
  children,
  onClose,
}: {
  children?: React.ReactNode;
  onClose?: () => void;
}) {
  const ref = useRef<HTMLDialogElement>(null!);
  useEffect(() => {
    ref.current.showModal();
  }, []);
  const onClick = (e: React.MouseEvent<HTMLDialogElement, MouseEvent>) => {
    const rect = ref.current.getBoundingClientRect();
    const clickedInDialog =
      rect.top <= e.clientY &&
      e.clientY <= rect.top + rect.height &&
      rect.left <= e.clientX &&
      e.clientX <= rect.left + rect.width;
    if (!clickedInDialog) onClose?.();
  };
  return (
    <dialog className={styles.Dialog} onClick={onClick} ref={ref}>
      {children}
    </dialog>
  );
}

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
