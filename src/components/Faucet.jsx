import React, { useState, useEffect } from 'react';
import { Wallet, Copy, ExternalLink, ChevronDown, ChevronUp, AlertCircle } from 'lucide-react';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { web3FromSource } from '@polkadot/extension-dapp';

const FAUCET_AMOUNT = '1000000000000';
const RPC_URL = 'wss://dev.qfnetwork.xyz/';

const Step = ({ number, title, children, isOpen, toggle }) => (
  <div className="border rounded-xl px-5 py-4 bg-white">
    <button
      onClick={toggle}
      className="w-full flex items-center justify-between font-medium text-lg"
    >
      <span className="flex items-center text-left font-karla font-semibold gap-4">
        <span className="flex items-center justify-center w-10 h-10 rounded-full bg-[#D9D9D9] ">
          {number}
        </span>
        {title}
      </span>
      <div className={`transform transition-transform duration-300 ${isOpen ? 'rotate-180' : ''}`}>
        <ChevronDown className="w-5 h-5" />
      </div>
    </button>
    <div
      className={`grid transition-all duration-300 ease-in-out ${
        isOpen ? 'grid-rows-[1fr] opacity-100' : 'grid-rows-[0fr] opacity-0'
      }`}
    >
      <div className="overflow-hidden">
        <div className="mt-4">{children}</div>
      </div>
    </div>
  </div>
);

const WalletStep = ({ onComplete }) => {
  const [hasExtension, setHasExtension] = useState(false);

  const checkExtension = () => {
    if (window.injectedWeb3?.['polkadot-js']) {
      setHasExtension(true);
      onComplete();
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex flex-col sm:flex-row items-start gap-4 mb-3">
        <img src="/api/placeholder/128/128" alt="Extension Store" className="rounded-lg w-32 h-32" />
        <div className="flex-1">
          <p className="mb-4">Install the Polkadot.js extension from your browser's store:</p>
          <div className="space-y-2">
            <a 
              href="https://chrome.google.com/webstore/detail/polkadot%7Bjs%7D-extension/mopnmbcafieddcagagdcbnhejhlodfdd"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-2 hover:text-black/70 underline decoration-1 underline-offset-4 "
            >
              Chrome Web Store <ExternalLink className="w-4 h-4" />
            </a>
            <a 
              href="https://addons.mozilla.org/en-US/firefox/addon/polkadot-js-extension/"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-2 hover:text-black/70 underline decoration-1 underline-offset-4"
            >
              Firefox Add-ons <ExternalLink className="w-4 h-4" />
            </a>
          </div>
        </div>
      </div>
      <button 
        onClick={checkExtension}
        className="w-full px-2 py-3 bg-[#777777] text-white font-karla font-semibold rounded-md hover:bg-[#676767] transition-colors duration-200"
      >
        I've installed the extension
      </button>
      {hasExtension && (
        <div className="p-3 bg-[#00C24810] text-[#01ab40] rounded-md flex items-center gap-2">
          Extension detected! ✓
        </div>
      )}
    </div>
  );
};

const AccountStep = ({ onComplete }) => {
  const [hasAccount, setHasAccount] = useState(false);

  return (
    <div className="space-y-4">
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
      <div className="flex gap-4 items-center">
        <button 
          onClick={() => {
            setHasAccount(true);
            onComplete();
          }}
          className="flex-1 px-2 py-3 bg-[#777777] text-white font-karla font-semibold rounded-md hover:bg-[#676767] transition-colors duration-200"
        >
          I've created my account
        </button>
      </div>
      {hasAccount && (
        <div className="p-3 text-[#01ab40] bg-[#00C24810] rounded-md flex items-center gap-2">
          Account created! ✓
        </div>
      )}
    </div>
  );
};

const FaucetStep = () => {
    const [account, setAccount] = useState(null);
    const [status, setStatus] = useState('');
    const [loading, setLoading] = useState(false);
    const [balance, setBalance] = useState(null);
    const [api, setApi] = useState(null);
  
    // Initialize API connection
    useEffect(() => {
      const initApi = async () => {
        try {
          const wsProvider = new WsProvider(RPC_URL);
          const api = await ApiPromise.create({ provider: wsProvider });
          setApi(api);
        } catch (error) {
          setStatus('Failed to connect to network: ' + error.message);
        }
      };
  
      initApi();
      return () => {
        if (api) {
          api.disconnect();
        }
      };
    }, []);
  
    // Subscribe to balance updates
    useEffect(() => {
      if (!api || !account) return;
  
      let unsubscribe;
  
      const subscribeBalance = async () => {
        unsubscribe = await api.query.system.account(account.address, ({ data: { free: currentBalance } }) => {
          setBalance(currentBalance.toString());
        });
      };
  
      subscribeBalance();
  
      return () => {
        if (unsubscribe) {
          unsubscribe();
        }
      };
    }, [api, account]);
  
    const connectWallet = async () => {
      try {
        setLoading(true);
        setStatus('Connecting to wallet...');
  
        // Enable extension
        const injected = await window.injectedWeb3['polkadot-js'].enable();
        const accounts = await injected.accounts.get();
  
        if (accounts.length > 0) {
          setAccount(accounts[0]);
          setStatus('Wallet connected!');
        } else {
          setStatus('No accounts found. Please create one first.');
        }
      } catch (err) {
        setStatus('Failed to connect: ' + err.message);
      } finally {
        setLoading(false);
      }
    };
  
    const requestTokens = async () => {
      if (!account || !api) return;
      
      try {
        setLoading(true);
        setStatus('Requesting tokens...');
  
        // Get the extension injector
        const injector = await web3FromSource(account.meta.source);
  
        // Make the actual faucet request
        const transfer = api.tx.faucet.requestTokens();
  
        // Sign and send the transaction
        await transfer.signAndSend(
            account.address,
            { signer: injector.signer },
            ({ status: txStatus, events = [] }) => {
              if (txStatus.isInBlock) {
                setStatus(`Transaction included in block ${txStatus.asInBlock}`);
              } else if (txStatus.isFinalized) {
                // Find transfer event and get amount
                events.forEach(({ event: { method, section, data } }) => {
                  if (section === 'balances' && method === 'Transfer') {
                    const [, , amount] = data;
                    setStatus(`Received ${amount.toString() / 1e12} tokens!`);
                  }
                });
                setLoading(false);
              }
            }
          );
        } catch (err) {
          console.error('Request tokens error:', err);
          console.log('Account object:', account);
          setStatus('Failed to request tokens: ' + err.message);
          setLoading(false);
        }
      };
  
    const formatBalance = (balance) => {
      if (!balance) return '0';
      return (parseInt(balance) / 1e12).toFixed(4);
    };
    const copyAddress = () => {
      if (account?.address) {
        navigator.clipboard.writeText(account.address);
        setStatus('Address copied!');
      }
    };
  
    return (
      <div className="space-y-4">
        {!account ? (
          <button
            onClick={connectWallet}
            disabled={loading}
            className="w-full flex items-center justify-center gap-4 px-2 py-3 bg-[#777777] text-white font-karla font-semibold rounded-md hover:bg-[#676767] transition-colors duration-200 disabled:opacity-50"
          >
            <Wallet className="w-6 h-6" />
            Connect Wallet
          </button>
        ) : (
          <div className="space-y-4">
            <div className="flex items-center justify-between p-3 bg-gray-50 rounded-md">
              <div className="truncate flex-1 font-mono text-sm">{account.address}</div>
              <button
                onClick={copyAddress}
                className="ml-2 text-gray-500 hover:text-gray-700"
              >
                <Copy className="w-5 h-5" />
              </button>
            </div>
  
            <div className="p-3 bg-gray-50 rounded-md">
              <div className="text-sm text-gray-600">Balance:</div>
              <div className="text-lg font-medium">
                {formatBalance(balance)} tokens
              </div>
            </div>
  
            <button
              onClick={requestTokens}
              disabled={loading}
              className="w-full p-2 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:opacity-50"
            >
              {loading ? 'Processing...' : 'Request Tokens'}
            </button>
          </div>
        )}
  
        {status && (
          <div className={`p-3 rounded-md text-sm flex items-center gap-2 ${
            status.includes('Failed') ? 'bg-[#C3230B20] text-[#C3230B]' : 'bg-[#A0BECC50] text-blue-700'
          }`}>
            <AlertCircle className="w-4 h-4" />
            {status} Failed
          </div>
        )}
      </div>
    );
  };

const Faucet = () => {
  const [openStep, setOpenStep] = useState(1);
  const [stepsCompleted, setStepsCompleted] = useState({
    1: false,
    2: false
  });

  const completeStep = (step) => {
    setStepsCompleted(prev => ({
      ...prev,
      [step]: true
    }));
    setOpenStep(step + 1);
  };

  return (
    <div className="relative p-6 pt-10 sm:pt-10 pb-20">
      <div className="text-center relative z-[1] mb-8 max-w-2xl mx-auto">
        <h1 className="text-3xl sm:text-5xl font-bold mb-2">QF devnet faucet</h1>
        <p className="font-light text-xs sm:text-base">Follow these steps to get started with <br /> test tokens</p>
      </div>

      <div className="space-y-4 relative z-[1] max-w-2xl mx-auto">
        <Step 
          number="1"
          title="Install Wallet Extension"
          isOpen={openStep === 1}
          toggle={() => setOpenStep(openStep === 1 ? null : 1)}
        >
          <WalletStep onComplete={() => completeStep(1)} />
        </Step>

        <Step 
          number="2"
          title="Create Account"
          isOpen={openStep === 2}
          toggle={() => setOpenStep(openStep === 2 ? null : 2)}
        >
          <AccountStep onComplete={() => completeStep(2)} />
        </Step>

        <Step 
          number="3"
          title="Request Tokens"
          isOpen={openStep === 3}
          toggle={() => setOpenStep(openStep === 3 ? null : 3)}
        >
          <FaucetStep />
        </Step>
      </div>


<div className="h-[1px] bg-[#DCDCDC] mt-8 w-[90%] mx-auto"></div>
      <div className="mt-8 p-5 relative z-[1] bg-white max-w-2xl mx-auto rounded-2xl border border-black">
        <h3 className="font-semibold mb-2 text-xl">Network Information</h3>
        <div className="text-sm">
          <p><strong>RPC Endpoint:</strong> {RPC_URL}</p>
          <p><strong>Token Amount per Request:</strong> {parseInt(FAUCET_AMOUNT) / 1e12} tokens</p>
        </div>
      </div>

      <img src="/circle.webp" alt="" className='absolute md:top-0 top-16 w-[35%] left-0 z-[0] md:w-[25%] max-w-sm' />
      <div className='h-[40%] bg-gradient-to-b from-[#C3230B] top-1/2 to-[#D6AE10] fixed right-0 w-3'></div>
    </div>
  );
};

export default Faucet;
