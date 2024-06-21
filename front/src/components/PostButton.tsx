import { useState } from "react";
import { useMutation } from "urql";

import { graphql } from "../gql";
import { Dialog } from "./Dialog";
import { setNote } from "../hashChanger";

export function PostButton() {
  const [isProcessing, setIsProcessing] = useState(false);
  const [showPostForm, setShowPostForm] = useState(false);
  const [, post] = useMutation(postMutation);
  const [text, setText] = useState("");
  const [postType, setPostType] = useState<"note" | "chat">("note");

  const submit = async () => {
    if (!text)
      return;
    setIsProcessing(true);
    const res = await post({ content: text });
    if (res.error || !res.data?.post.id) {
      alert("Failed to post.");
      return
    }

    setText("");
    setShowPostForm(false);
    setIsProcessing(false);
    setNote(res.data.post.id);
  };

  return (
    <>
      <button onClick={() => setShowPostForm(true)}>+</button>

      {showPostForm && (
        <Dialog onClose={() => setShowPostForm(false)}>
          <div className="flex flex-col items-center gap-4">
            <div className="flex gap-2">
              <button className="" data-active={postType === "note"} onClick={() => setPostType("note")}>Note</button>
              <button className="" data-active={postType === "chat"} onClick={() => setPostType("chat")}>Chat</button>
            </div>
            <textarea className="w-96 h-40 border rounded p-2" value={text} onChange={(e) => setText(e.target.value)} />
            <button onClick={submit} disabled={isProcessing}>Post</button>
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
