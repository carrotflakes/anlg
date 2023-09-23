import { Provider } from 'urql'
import { client } from '../../urql'
import { Test } from '../Test'
import styles from './index.module.scss'

function App() {
  return (
    <Provider value={client}>
      <div className={styles.App}>
        <h2>anlg</h2>
        <Test />
      </div>
    </Provider>
  )
}

export default App
