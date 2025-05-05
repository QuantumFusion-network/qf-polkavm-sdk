//
//
//

import React, {useState, useEffect, useRef} from 'react';
import {Wallet} from 'lucide-react';
import {ApiPromise, WsProvider} from '@polkadot/api';
import {web3Accounts, web3Enable, web3FromAddress} from '@polkadot/extension-dapp';

import {WalletStep} from "./WalletStep.jsx";
import {ContractStep} from "./ContractStep.jsx";
import {AccountStep} from "./AccountStep.jsx";
import {ContractMethods} from "./ContractMethods.jsx";
import {Logs} from "./Logs.jsx";

const RPC_URL = 'wss://dev.qfnetwork.xyz/socket';

const Faucet = () => {

  const [account, setAccount] = useState(null);
  const [injector, setInjector] = useState(null);
  const [api, setApi] = useState(null);
  const [contractAddress, setContractAddress] = useState(null);
  const [contractMethods, setContractMethods] = useState([]);
  const [logs, setLogs] = useState([]);

  const [rpc, setRpc] = useState(RPC_URL)
  const rpcRef = useRef()

  const onSetRpc = () => {
    try {
      if (rpcRef.current?.value) {
        setRpc(rpcRef.current.value)
      }
      setLogs([])
    } catch (e) {
      console.log(e)
      setLogs([...logs, 'Error to connect'])
    }
  }

  const connectWallet = async () => {
    await web3Enable("DApp");

    const accounts = await web3Accounts();
    if (!accounts.length) throw new Error("No accounts found");

    const account = accounts[0];
    const injector = await web3FromAddress(account.address);

    setInjector(injector)
    setAccount(account)
  };

  const connectApi = async (rpc) => {
    try {
      const wsProvider = new WsProvider(rpc);
      const api = await ApiPromise.create({provider: wsProvider});
      setApi(api);
      setLogs([])
    } catch (error) {
      setLogs([...logs, 'Failed to connect to network: ' + error.message]);
    }
  };

  useEffect(() => {


    connectApi();

    return () => {
      if (api) {
        api.disconnect();
      }
    };
  }, [rpc]);


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
            <p><strong>RPC Endpoint:</strong> {rpc}</p>
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
                setContractMethods={setContractMethods}

              />
            )}

            {!!accountAddress && !!contractAddress && (
              <ContractMethods
                injector={injector}
                account={account}
                api={api} contractMethods={contractMethods || []}
                contractAddress={contractAddress}
              />
            )}
          </div>
        </div>
      </div>


      <div className="h-[1px] bg-[#DCDCDC] mt-8 w-[90%] mx-auto"></div>
      <div className="mt-8 p-5 relative z-[1] bg-white max-w-2xl mx-auto rounded-2xl border border-black">
        <h3 className="font-semibold mb-2 text-xl">Network Information</h3>
        <div className="text-sm">
          <p><strong>RPC Endpoint:</strong> {rpc}</p>
        </div>

        <input ref={rpcRef} placeholder={rpc} className="mt-4 w-full py-2 px-3 border rounded-lg" type="text"/>
        <button
          className="mt-2 flex items-center justify-center py-2 px-3 font-karla font-semibold rounded-md transition-colors duration-200 p-3 text-[#fff] hover:bg-[#00c2489c] bg-[#00c248c9]"
          onClick={onSetRpc}>
          Submit
        </button>
        <div className="mt-4">
          <Logs logs={logs}/>
        </div>
      </div>

      <img src="/circle.webp" alt="" className='absolute md:top-0 top-16 w-[35%] left-0 z-[0] md:w-[25%] max-w-sm'/>
      <div className='h-[40%] bg-gradient-to-b from-[#C3230B] top-1/2 to-[#D6AE10] fixed right-0 w-3'></div>
    </div>
  );
};

export default Faucet;
