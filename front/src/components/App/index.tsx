import { Provider } from 'urql'
import { client } from '../../urql'
import { Notes } from '../Notes'
import styles from './index.module.scss'
import { PostButton } from '../PostButton'
import { useEffect, useState } from 'react'

function App() {
  const [state, setState] = useState(parseHash(location.hash));
  useEffect(() => {
    const onHashChange = () => {
      setState(parseHash(location.hash))
    }
    window.addEventListener('hashchange', onHashChange)
    return () => {
      window.removeEventListener('hashchange', onHashChange)
    }
  })

  return (
    <Provider value={client}>
      <div className={styles.App}>
        <header>
          <h1>anlg</h1>
          <PostButton />
        </header>
        <Notes noteId={state.noteId} />
      </div>
    </Provider>
  )
}

export default App

function parseHash(hash: string) {
  const matchWithNotes = hash.match(/#\/notes\/([^/]+)/)
  if (matchWithNotes) {
    const [, id] = matchWithNotes
    return {
      noteId: id
    }
  }
  return {
    noteId: null
  }
}
