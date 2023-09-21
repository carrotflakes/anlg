import { Provider } from 'urql'
import { client } from '../../urql'
import { Test } from '../Test'
import styles from './index.module.scss'

function App() {
  return (
    <Provider value={client}>
      <div className={styles.App}>
        anlg
        <Test />
      </div>
    </Provider>
  )
}

export default App
