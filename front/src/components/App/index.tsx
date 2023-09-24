import { Provider } from 'urql'
import { client } from '../../urql'
import { Notes } from '../Notes'
import styles from './index.module.scss'
import { PostButton } from '../PostButton'

function App() {
  return (
    <Provider value={client}>
      <div className={styles.App}>
        <header>
          <h1>anlg</h1>
          <PostButton />
        </header>
        <Notes />
      </div>
    </Provider>
  )
}

export default App
