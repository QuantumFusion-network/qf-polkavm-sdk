import React from 'react';
import { BrowserRouter as Router, Routes, Route, useParams } from 'react-router-dom';
import {Deploy} from "./modules/deploy/index.jsx";
import {Connect} from "./modules/connect/index.jsx";
import {WalletProvider} from "./hooks/useAccounts.jsx";
import {ApiProvider} from "./hooks/useApi.jsx";
import {RPC_URL} from "./utils/constants.js";
import {Contracts} from "./modules/contracts/index.jsx";
import {Contract} from "./modules/contract/index.jsx";
import {RpcNodesProvider, useRpcNodes} from "./hooks/useRpcNodes.jsx";


function AccountPage() {
  return <div>👤 Account Page</div>;
}


export default function AppRouter() {
// 1 case:
// адрес смарта
// hex строку
// gas limit
// gas price
// one more


  const {
    selectedNode
  } = useRpcNodes()

  return (
    <ApiProvider rpcUrl={selectedNode.url}>
      <WalletProvider>
        <Router>
          <Routes>
            <Route path="/connect" element={<Connect />} />
            <Route path="/" element={<Deploy />} />
            <Route path="/account" element={<AccountPage />} />
            <Route path="/contract/:contractAddress" element={<Contract />} />
            <Route path="/contracts" element={<Contracts />} />
          </Routes>
        </Router>
      </WalletProvider>
    </ApiProvider>

  );
}

