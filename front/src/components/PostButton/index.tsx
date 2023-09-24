import { useState } from "react";
import { useMutation } from "urql";

import styles from "./index.module.scss";
import { graphql } from "../../gql";
import { Dialog } from "../Dialog";

export function PostButton() {
  const [showPostForm, setShowPostForm] = useState(false);
  const [, post] = useMutation(postMutation);
  const [text, setText] = useState("");

  const submit = async () => {
    await post({ content: text });
    // refresh({ requestPolicy: "network-only" });
    setText("");
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
