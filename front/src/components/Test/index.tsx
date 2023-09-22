import { useMutation, useQuery } from 'urql'
import { graphql } from '../../gql'
import { useState } from 'react'

export function Test() {
  const [res] = useQuery({ query: aQuery })
  const [notesRes] = useQuery({ query: notesQuery })
  const [, post] = useMutation(postMutation)
  const [text, setText] = useState('')

  return (
    <div>
      Test
      {res.data?.add}
      {notesRes.data?.notes.map((note: { content: string, createdAt: string }) => (
        <div>{note.content} - {note.createdAt}</div>
      ))}
      <input value={text} onChange={e => setText(e.target.value)} />
      <button onClick={() => post({ content: text })}>post</button>
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
