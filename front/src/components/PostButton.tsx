import { useState } from "react";
import { useMutation } from "urql";

import { graphql } from "../gql";
import { Dialog } from "./Dialog";
import { setNote } from "../hashChanger";
import { Button } from "./Button";

export function PostButton() {
  const [isProcessing, setIsProcessing] = useState(false);
  const [showPostForm, setShowPostForm] = useState(false);
  const [, post] = useMutation(postMutation);
  const [, newChat] = useMutation(newChatMutation);
  const [text, setText] = useState("");
  const [postType, setPostType] = useState<"note" | "chat">("note");

  const submit = async () => {
    if (!text)
      return;

    if (postType === "note") {
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
    } else {
      setIsProcessing(true);
      const res = await newChat({ content: text });
      if (res.error || !res.data?.newChat.id) {
        alert("Failed to post.");
        return
      }

      setText("");
      setShowPostForm(false);
      setIsProcessing(false);
    }
  };

  return (
    <>
      <Button onClick={() => setShowPostForm(true)}>+</Button>

      {showPostForm && (
        <Dialog onClose={() => setShowPostForm(false)}>
          <div className="flex flex-col items-center gap-4">
            <div className="flex gap-2">
              <Button onClick={() => setPostType("note")} selected={postType === "note"}>Note</Button>
              <Button onClick={() => setPostType("chat")} selected={postType === "chat"}>Chat</Button>
            </div>
            <textarea className="w-96 h-40 border rounded-sm p-2" value={text} onChange={(e) => setText(e.target.value)} />
            <Button onClick={submit} disabled={isProcessing}>Post</Button>
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

const newChatMutation = graphql(`
  mutation newChat($content: String!) {
    newChat(content: $content) {
      id
    }
  }
`);
