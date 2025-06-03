import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.jsx'
import {RpcNodesProvider} from "./hooks/useRpcNodes.jsx";

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <RpcNodesProvider>
      <App />
    </RpcNodesProvider>

  </StrictMode>,
)
