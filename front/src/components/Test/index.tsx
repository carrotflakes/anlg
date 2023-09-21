import { useQuery } from 'urql'
import { graphql } from '../../gql'

export function Test() {
  const [res] = useQuery({ query: aQuery })
  const [notesRes] = useQuery({ query: notesQuery })

  return (
    <div>
      Test
      {res.data?.add}
      {notesRes.data?.notes.map((note: { content: string, createdAt: string }) => (
        <div>{note.content} - {note.createdAt}</div>
      ))}
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
