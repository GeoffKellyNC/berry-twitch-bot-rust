import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './styles.css'
import { BrowserRouter as Router  } from 'react-router-dom'

import { Provider } from 'react-redux';
import { applyMiddleware, compose, legacy_createStore as createStore } from 'redux';
import rootReducer from './store/root.reducer.ts';
import { thunk }  from 'redux-thunk';


declare global {
    interface Window {
      __REDUX_DEVTOOLS_EXTENSION_COMPOSE__?: typeof compose;
    }
}

const isLocal = import.meta.env.VITE_REACT_APP_LOCAL_MODE;
const composeEnhancers = (isLocal && window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__) || compose;


const store = createStore(
    rootReducer, 
    composeEnhancers(applyMiddleware(thunk))
);


ReactDOM.createRoot(document.getElementById('root')!).render(
  <Provider store={store}>
    <Router>
      <App />
    </Router>
  </Provider>
  )
