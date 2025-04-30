import React, {useEffect, useState} from "react";
import {ExternalLink} from "lucide-react";

export const WalletStep = () => {
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
          <p className="mb-4">Install the Polkadot.js extension from your browser&#39;s store:</p>
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
