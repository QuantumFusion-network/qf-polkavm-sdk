//
//
//

import React, {useState, useEffect} from 'react';
import {Wallet} from 'lucide-react';
import {ApiPromise, WsProvider} from '@polkadot/api';
import {web3Accounts, web3Enable, web3FromAddress} from '@polkadot/extension-dapp';

import {WalletStep} from "./WalletStep.jsx";
import {ContractStep} from "./ContractStep.jsx";
import {AccountStep} from "./AccountStep.jsx";
import {ContractMethods} from "./ContractMethods.jsx";
const RPC_URL = 'wss://dev.qfnetwork.xyz/socket';

const Faucet = () => {

  const [account, setAccount] = useState(null);
  const [injector, setInjector] = useState(null);
  const [api, setApi] = useState(null);
  const [contractAddress, setContractAddress] = useState("5H2xQFnmx73YMXmYPjUprqZe5Cuaa3HzDSsZMsqChstwBYwC");

  const connectWallet = async () => {
    await web3Enable("DApp");

    const accounts = await web3Accounts();
    if (!accounts.length) throw new Error("No accounts found");

    const account = accounts[0];
    const injector = await web3FromAddress(account.address);

    setInjector(injector)
    setAccount(account)
  };

  useEffect(() => {
    const initApi = async () => {
      try {
        const wsProvider = new WsProvider(RPC_URL);
        const api = await ApiPromise.create({provider: wsProvider});
        setApi(api);
      } catch (error) {
        console.log('Failed to connect to network: ' + error.message);
      }
    };

    initApi();
    return () => {
      if (api) {
        api.disconnect();
      }
    };
  }, []);


  const isReady = api?._isReady
  const accountAddress = account?.address;

  if (!isReady) {
    return (
      <div className="relative p-6 pt-10 sm:pt-10 pb-20">
        <div className="space-y-4 relative z-[1] max-w-2xl mx-auto">
          <div className='rounded-lg border bg-[#ffffff] p-4'>
            <div className="space-y-4 text-center">
              <h1>Connectig to RPC ...</h1>
            </div>
          </div>
        </div>
        <div className="mt-8 p-5 relative z-[1] bg-white max-w-2xl mx-auto rounded-2xl border border-black">
          <h3 className="font-semibold mb-2 text-xl">Network Information</h3>
          <div className="text-sm">
            <p><strong>RPC Endpoint:</strong> {RPC_URL}</p>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="relative p-6 pt-10 sm:pt-10 pb-20">
      <div className="text-center relative z-[1] mb-8 max-w-2xl mx-auto">
        <h1 className="text-3xl sm:text-5xl font-bold mb-2">QF devnet faucet</h1>
        <p className="font-light text-xs sm:text-base">Follow these steps to get started with <br/> test tokens</p>
      </div>

      <div className="space-y-4 relative z-[1] max-w-2xl mx-auto">
        <div className='rounded-lg border bg-[#ffffff] p-4'>
          <div className="space-y-4">
            <WalletStep/>
            <AccountStep account={accountAddress}/>

            {!accountAddress && (
              <button
                onClick={connectWallet}
                className="w-full flex items-center justify-center gap-4 px-2 py-3 bg-[#777777] text-white font-karla font-semibold rounded-md hover:bg-[#676767] transition-colors duration-200 disabled:opacity-50"
              >
                <Wallet className="w-6 h-6"/>
                Connect Wallet
              </button>
            )}

            {!!accountAddress && (
              <ContractStep
                injector={injector}
                account={account}
                api={api}
                setContractAddress={setContractAddress}
              />
            )}

            {!!contractAddress && <ContractMethods api={api} contractAddress={contractAddress}/>}
          </div>
        </div>
      </div>


      <div className="h-[1px] bg-[#DCDCDC] mt-8 w-[90%] mx-auto"></div>
      <div className="mt-8 p-5 relative z-[1] bg-white max-w-2xl mx-auto rounded-2xl border border-black">
        <h3 className="font-semibold mb-2 text-xl">Network Information</h3>
        <div className="text-sm">
          <p><strong>RPC Endpoint:</strong> {RPC_URL}</p>
        </div>
      </div>

      <img src="/circle.webp" alt="" className='absolute md:top-0 top-16 w-[35%] left-0 z-[0] md:w-[25%] max-w-sm'/>
      <div className='h-[40%] bg-gradient-to-b from-[#C3230B] top-1/2 to-[#D6AE10] fixed right-0 w-3'></div>
    </div>
  );
};

export default Faucet;
