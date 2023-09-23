import { useMutation, useQuery } from 'urql'
import { graphql } from '../../gql'
import { useState } from 'react'

export function Test() {
  const [res] = useQuery({ query: aQuery })
  const [notesRes, refresh] = useQuery({ query: notesQuery })
  const [, post] = useMutation(postMutation)
  const [, deleteMut] = useMutation(deleteMutation)
  const [text, setText] = useState('')

  const deleteNote = async (id: string) => {
    await deleteMut({ id })
    refresh({ requestPolicy: 'network-only' })
  }

  const submit = async () => {
    await post({ content: text });
    refresh({ requestPolicy: 'network-only' })
    setText('')
  };

  return (
    <div>
      Test
      {res.data?.add}
      {notesRes.data?.notes.map((note: { id: string, content: string, createdAt: string }) => (
        <div key={note.id}>{note.content} - {new Date(note.createdAt).toISOString()} - <button onClick={() => deleteNote(note.id)}>x</button></div>
      ))}
      <textarea value={text} onChange={e => setText(e.target.value)} />
      <button onClick={submit}>post</button>
    </div>
  )
}

const aQuery = graphql(`
  query a {
    add(a: 1, b: 2)
  }
`)

const notesQuery = graphql(`
  query notes {
    notes {
      id
      content
      createdAt
    }
  }
`)

const postMutation = graphql(`
  mutation post($content: String!) {
    post(content: $content)
  }
`)

const deleteMutation = graphql(`
  mutation delete($id: String!) {
    delete(noteId: $id)
  }
`)
