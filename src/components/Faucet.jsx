//
//
//

import React, {useState, useEffect} from 'react';
import {Wallet, ExternalLink} from 'lucide-react';
import {ApiPromise, WsProvider} from '@polkadot/api';
import {web3Accounts, web3Enable, web3FromAddress} from '@polkadot/extension-dapp';
import {hexToU8a, isHex, u8aToHex, u8aToString} from '@polkadot/util';
import {Output} from "./Output.jsx";
import {DecodedInspect} from "./DecodedInspect.jsx";
import {Logs} from "./Logs.jsx";

const BYTE_STR_0 = '0'.charCodeAt(0);
const BYTE_STR_X = 'x'.charCodeAt(0);
const STR_NL = '\n';
const NOOP = () => undefined;

const RPC_URL = 'wss://dev.qfnetwork.xyz/socket';

const WalletStep = () => {
  const [hasExtension, setHasExtension] = useState(false);

  useEffect(() => {
    if (window.injectedWeb3?.['polkadot-js']) {
      setHasExtension(true);
    }
  }, [])

  return (
    <div className="space-y-4">
      <div className="flex flex-col sm:flex-row items-start gap-4 mb-3">

        <div className="flex-1">
          <p className="mb-4">Install the Polkadot.js extension from your browser's store:</p>
          <div className="space-y-2">
            <a
              href="https://chrome.google.com/webstore/detail/polkadot%7Bjs%7D-extension/mopnmbcafieddcagagdcbnhejhlodfdd"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-2 hover:text-black/70 underline decoration-1 underline-offset-4 "
            >
              Chrome Web Store <ExternalLink className="w-4 h-4"/>
            </a>
            <a
              href="https://addons.mozilla.org/en-US/firefox/addon/polkadot-js-extension/"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-2 hover:text-black/70 underline decoration-1 underline-offset-4"
            >
              Firefox Add-ons <ExternalLink className="w-4 h-4"/>
            </a>
          </div>
        </div>
      </div>
      {hasExtension && (
        <div className="p-3 bg-[#00C24810] text-[#01ab40] rounded-md flex items-center gap-2">
          Extension detected! ✓
        </div>
      )}
    </div>
  );
};

function convertResult (result) {
  const data = new Uint8Array(result);

  // this converts the input (if detected as hex), via the hex conversion route
  if (data[0] === BYTE_STR_0 && data[1] === BYTE_STR_X) {
    let hex = u8aToString(data);

    while (hex.endsWith(STR_NL)) {
      hex = hex.substring(0, hex.length - 1);
    }

    if (isHex(hex)) {
      return hexToU8a(hex);
    }
  }

  return data;
}

function extract(isCall, extrinsic, payload) {
  if (!extrinsic) {
    return ['0x', '0x', null];
  }

  const u8a = extrinsic.method.toU8a();
  let inspect = isCall
    ? extrinsic.method.inspect()
    : extrinsic.inspect();

  if (payload) {
    const prev = inspect;

    inspect = payload.inspect();
    inspect.inner?.map((entry, index) => {
      if (index === 0) {
        // replace the method inner
        entry.inner = prev.inner;
        entry.outer = undefined;
      }

      return entry;
    });
  }

  return [
    u8aToHex(u8a),
    extrinsic.registry.hash(u8a).toHex(),
    inspect
  ];
}


const AccountStep = ({account}) => {
  return (
    <div className="space-y-4">
      {!account && (
        <div className="rounded-lg border bg-[#F5F4F4] p-4">
          <h3 className="font-medium font-bold mb-2">Create a new account:</h3>
          <ol className="list-decimal pl-4 space-y-2">
            <li>Click the Polkadot{'{.js}'} extension icon in your browser</li>
            <li>Click the big plus (+) button</li>
            <li>Select "Create new account"</li>
            <li>
              <strong className="text-[#C3230B]">IMPORTANT:</strong> Save your seed phrase securely!
            </li>
            <li>Set a descriptive name and password</li>
          </ol>
        </div>
      )}
      {account ? (
        <div className="p-3 text-[#01ab40] bg-[#00C24810] rounded-md flex items-center gap-2">
          <p>
            Account connected: {account}
          </p>

        </div>
      ) : (
        <div className="p-3 text-[#e91e63] bg-[#e91e631f] rounded-md flex items-center gap-2">
          Account not connected
        </div>
      )}

    </div>
  );
};


const ContractStep = ({api, account, injector, setInfo, info, setExtrinsic, extrinsic, onSubmit, logs}) => {

  const onUpload = async (e) => {
    const file = e.target.files?.[0];
    if (file && api && account && injector) {
      try {
        const reader = new FileReader();

        reader.onabort = NOOP;
        reader.onerror = NOOP;

        reader.onload = ({ target }) => {
          if (target?.result) {
            const name = file.name;
            const data = convertResult(target.result);


            console.log({name, data})

            const extrinsic = api.tx.qfPolkaVM.upload(data);

            console.log({extrinsic})

            setInfo(extract(true, extrinsic, undefined))
            setExtrinsic(extrinsic)
          }
        };

        reader.readAsArrayBuffer(file);

      } catch (error) {
        console.error("Upload error:", error);
        throw error;
      }
    }
  }


  return (
    <div className="space-y-4">
      {!info && (
        <input type="file" accept=".polkavm" onChange={onUpload}/>
      )}

      <h1>Submit the following extrinsic:</h1>

      {!!info?.[0] && <Output label={"encoded call data"} isTrimmed value={info[0]}/>}
      {!!info?.[1] && <Output label={"encoded call hash"} value={info[1]}/>}
      {!!info?.[0] && !!info?.[2] && <DecodedInspect hex={info[0]} inspect={info[2]}/>}

      {!!extrinsic && (
        <button
          onClick={onSubmit}
          className="w-full flex items-center justify-center gap-4 px-2 py-3 font-karla font-semibold rounded-md transition-colors duration-200 p-3 text-[#fff] hover:bg-[#00c2489c] bg-[#00c248c9]  rounded-md flex items-center gap-2">Submit</button>
      )}

      <Logs logs={logs || []}/>
    </div>
  )
}

const Faucet = () => {

  const [account, setAccount] = useState(null);
  const [injector, setInjector] = useState(null);
  const [info, setInfo] = useState(null)
  const [deployLogs, setDeployLogs] = useState([])

  const [api, setApi] = useState(null);
  const [extrinsic, setExtrinsic] = useState(null);

  const connectWallet = async () => {
    await web3Enable("DApp");

    const accounts = await web3Accounts();
    if (!accounts.length) throw new Error("No accounts found");

    const account = accounts[0];
    const injector = await web3FromAddress(account.address);

    setInjector(injector)
    setAccount(account)
  };

  const onSend = async () => {
    if (!extrinsic) {
      return
    }

    await extrinsic.signAndSend(account.address, {signer: injector.signer}, ({status, events}) => {
      console.log("📡 Status update:", status.type);

      if (status.isInBlock) {
        console.log("📦 Included in block:", status.asInBlock.toHex());
      }

      if (status.isFinalized) {
        console.log("🎉 Finalized in block:", status.asFinalized.toHex());
      }

      if (events && events.length > 0) {
        console.log("📜 Events received:");
        events.forEach(({phase, event: {section, method, data}}, index) => {
          console.log(`  [${index}] ${section}.${method} (phase: ${phase.toString()})`);
          data.forEach((arg, i) => {
            console.log(`    └─ Arg[${i}]:`, arg.toHuman());
          });
        });
      } else {
        console.log("⚠️ No events received.");
      }

      // Detect success
      const success = events.find(({event}) => event.section === 'system' && event.method === 'ExtrinsicSuccess');
      if (success) {
        console.log("✅ Extrinsic executed successfully.");
      }

      // Detect failure and decode
      const failure = events.find(({event}) => event.section === 'system' && event.method === 'ExtrinsicFailed');
      if (failure) {
        const dispatchError = failure.event.data[0];
        if (dispatchError.isModule) {
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          console.error(`❌ Extrinsic failed: ${decoded.section}.${decoded.name}`);
          console.error(`📖 Reason: ${decoded.docs.join(' ')}`);
        } else {
          console.error("❌ Extrinsic failed with error:", dispatchError.toString());
        }
      }

      // Try to find contract upload event
      const uploadedEvent = events.find(
        ({event}) => event.section === "qfPolkaVM" && event.method === "ContractUploaded"
      );

      if (uploadedEvent) {
        const contractAddress = uploadedEvent.event.data[0].toString();
        console.log("📦 Contract uploaded at:", contractAddress);
      } else {
        console.warn("⚠️ qfPolkaVM.ContractUploaded event not found.");
      }
    });
  }

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
                deployLogs={deployLogs}
                onSubmit={onSend}
                extrinsic={extrinsic}
                setExtrinsic={setExtrinsic}
                info={info}
                setInfo={setInfo}
                injector={injector}
                account={account}
                api={api}
              />
            )}
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
