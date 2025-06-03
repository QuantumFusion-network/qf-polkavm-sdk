import React, { createContext, useContext, useEffect, useState } from 'react';
import { ApiPromise, WsProvider } from '@polkadot/api';
import toast, { Toaster } from 'react-hot-toast';

const ApiContext = createContext();
// filename
// exists contract
// fix connect wallet
// fix notifications

export const ApiProvider = ({ rpcUrl, children }) => {
  const [_rpcUrl, setRpcUrl] = useState(rpcUrl)

  console.log('rpcUrl ', rpcUrl)
  const [api, setApi] = useState(null);
  const [isReady, setIsReady] = useState(false);
  const [error, setError] = useState(null);

  useEffect(() => {
    let apiInstance = null;

    const connect = async () => {
      try {
        await api?.disconnect()
        setIsReady(false);
        const provider = new WsProvider(rpcUrl);
        apiInstance = await ApiPromise.create({ provider });

        setApi(apiInstance);
        setIsReady(true);
        toast.success(`Success connect to ${rpcUrl}!`)

      } catch (err) {
        toast.error(`Failed connect to ${rpcUrl}!`)

      }
    };

    void connect();
  }, [_rpcUrl, rpcUrl]);

  return (
    <ApiContext.Provider value={{ api, isReady, error }}>
      <Toaster/>
      {children}
    </ApiContext.Provider>
  );
};

export const useApi = () => {
  const context = useContext(ApiContext);
  if (!context) {
    throw new Error('useApi must be used within an ApiProvider');
  }
  return context;
};
