import React, {createContext, useContext, useState, useEffect, useRef} from 'react';
import { web3Enable, web3Accounts } from '@polkadot/extension-dapp';

const WalletContext = createContext();

export const WalletProvider = ({ children }) => {
  const [accounts, setAccounts] = useState([]);
  const [selected, setSelected] = useState(null);
  const [alreadyConnected, setAlreadyConnected] = useState(false);

  const extensionsRef = useRef()

  const connectWallet = async () => {
    const extensions = await web3Enable('DApp');
    if (!extensions.length) return;

    const _accounts = await web3Accounts();
    setAccounts(_accounts || []);
    if (_accounts[0]?.address) {
      setSelected(_accounts[0]);
      setAlreadyConnected(true);
      extensionsRef.current = extensions[0];
    }
  };

  const disconnectWallet = () => {
    extensionsRef.current.provider.disconnect().then(() => {
      setSelected(null);
      setAlreadyConnected(false);
    })
  }

  const selectAddress = (a) => setSelected(a)

  // useEffect(() => {
  //   void connectWallet();
  // }, []);

  return (
    <WalletContext.Provider value={{ accounts, selected, connectWallet, alreadyConnected, selectAddress, disconnectWallet }}>
      {children}
    </WalletContext.Provider>
  );
};

export const useWallet = () => {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error('useWalletContext must be used within a WalletProvider');
  }
  return context;
};
