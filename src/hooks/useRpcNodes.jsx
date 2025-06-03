import React, { createContext, useContext, useState } from 'react';

const LAST_CONNECTED_RPC = 'LAST_CONNECTED_RPC'
const RpcNodesContext = createContext();

export const RpcNodesProvider = ({ children }) => {
  const DEV_NODE_RPC = import.meta.env.VITE_DEV_NODE_RPC
  const TEST_NODE_RPC = import.meta.env.VITE_TEST_NODE_RPC
  const DEV_NODE_PARACHAIN_RPC = import.meta.env.VITE_DEV_NODE_PARACHAIN_RPC
  const TEST_NODE_PARACHAIN_RPC = import.meta.env.VITE_TEST_NODE_PARACHAIN_RPC

  const updateNode = (n) => {
    localStorage.setItem(LAST_CONNECTED_RPC, JSON.stringify(n))
    setSelectedNode(n)
  }

  const getNode = () => {
    const lastConnectedJSON = localStorage.getItem(LAST_CONNECTED_RPC)
    return  lastConnectedJSON ? JSON.parse(lastConnectedJSON) : {
      name: "Devnet chain",
      url: DEV_NODE_RPC
    }
  }

  const [selectedNode, setSelectedNode] = useState(getNode())


  const nodes = [
    {
      name: "QF Devnet",
      url: DEV_NODE_RPC
    },
    {
      name: "QF Testnet",
      url: TEST_NODE_RPC
    },
    {
      name: "QF Testnet Parachain (Paseo)",
      url: DEV_NODE_PARACHAIN_RPC
    },
    {
      name: "QF Devnet Parachain (Paseo)",
      url: TEST_NODE_PARACHAIN_RPC
    }
  ]

  return (
    <RpcNodesContext.Provider value={{nodes, getNode, updateNode, selectedNode}}>
      {children}
    </RpcNodesContext.Provider>
  );
};

export const useRpcNodes = () => {
  const context = useContext(RpcNodesContext);
  if (!context) {
    throw new Error('useWalletContext must be used within a WalletProvider');
  }
  return context;
};
