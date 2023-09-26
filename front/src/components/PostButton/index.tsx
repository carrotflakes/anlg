import { useState } from "react";
import { useMutation } from "urql";

import styles from "./index.module.scss";
import { graphql } from "../../gql";
import { Dialog } from "../Dialog";
import { setNote } from "../../hashChanger";

export function PostButton() {
  const [showPostForm, setShowPostForm] = useState(false);
  const [, post] = useMutation(postMutation);
  const [text, setText] = useState("");

  const submit = async () => {
    const res = await post({ content: text });
    if (res.error || !res.data?.post.id) {
      alert("Failed to post.");
      return
    }

    setText("");
    setShowPostForm(false);
    setNote(res.data.post.id);
  };

  return (
    <>
      <button onClick={() => setShowPostForm(true)}>+</button>

      {showPostForm && (
        <Dialog onClose={() => setShowPostForm(false)}>
          <div className={styles.postForm}>
            <textarea value={text} onChange={(e) => setText(e.target.value)} />
            <button onClick={submit}>Post</button>
          </div>
        </Dialog>
      )}
    </>
  )
}

const postMutation = graphql(`
  mutation post($content: String!) {
    post(content: $content) {
      id
    }
  }
`);
