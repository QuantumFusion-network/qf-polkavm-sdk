import React from "react";
import PropTypes from "prop-types";

export const AccountStep = ({account}) => {
  return (
    <div className="space-y-4">
      {!account && (
        <div className="rounded-lg border bg-[#F5F4F4] p-4">
          <h3 className="font-medium font-bold mb-2">Create a new account:</h3>
          <ol className="list-decimal pl-4 space-y-2">
            <li>Click the Polkadot{'{.js}'} extension icon in your browser</li>
            <li>Click the big plus (+) button</li>
            <li>Select &#34;Create new account&#34;</li>
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

AccountStep.propTypes = {
  account: PropTypes.shape({
    address: PropTypes.string.isRequired,
    meta: PropTypes.shape({
      name: PropTypes.string,
      source: PropTypes.string
    }),
    type: PropTypes.string // sr25519, ed25519 и т.д. (опционально)
  }).isRequired
};
