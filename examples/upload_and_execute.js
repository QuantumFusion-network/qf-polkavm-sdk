/*
 * Script for calling a smart contract. You could paste that it to https://portal.qfnetwork.xyz/#/js.
 */

// 1. Configure smart contract call.

// Chain configuration.
const DECIMALS = Math.pow(10, 18);
const WEIGHT = Math.pow(10, 9)

// Call origin. Make sure you have this account connected from the wallet.
const SENDER = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'; // Alice

// Call arguments.
const args = {
  dest: '0x2c6fc00458f198f46ef072e1516b83cd56db7cf5', // smart contract address
  value: 1 * DECIMALS, // amount to send with the call
  gasLimit: {
    refTime: 1 * WEIGHT, // gas limit ref time
    proofSize: 1 * WEIGHT, // gas limit proof size
  },
  storageDepositLimit: 1 * DECIMALS, // storage deposit limit
  data: '0xffffff00', // SCALE-encoded smart contract function arguments
};

// 2. Main execution flow.
console.log('Checking if account is mapped...');
const mappedAddress = await findMappedAccount();

if (mappedAddress) {
  console.log(`Account already mapped. Ethereum address: ${mappedAddress}`);
} else {
  console.log('Account not found in mapping entries');
  await mapAccount();

  const newMappedAddress = await findMappedAccount();
  if (newMappedAddress) {
    console.log(`Account mapped successfully. Ethereum address: ${newMappedAddress}`);
  } else {
    console.log('Account mapping may have failed - account not found in entries');
  }
}

// Execute the smart contract call.
const blockHash = await executeCall();
console.log('Contract execution completed successfully');
console.log(`blockHash: ${blockHash}`)

/*
 * Helpers.
 */

function buf2hex(buffer) { // buffer is an ArrayBuffer
  return [...new Uint8Array(buffer)]
    .map(x => x.toString(16).padStart(2, '0'))
    .join('');
}

async function findMappedAccount() {
  const allOriginalAccounts = await api.query.revive.originalAccount.entries();

  for (const [key, value] of allOriginalAccounts) {
    const substrateAccount = value.toString();
    if (substrateAccount === SENDER) {
      const ethereumAddressBytes = key.args[0];
      const ethereumAddress = `0x${buf2hex(ethereumAddressBytes.toU8a())}`;
      return ethereumAddress;
    }
  }
  return null;
}

async function mapAccount() {
  const mapAccountTx = api.tx.revive.mapAccount();

  return new Promise((resolve, reject) => {
    const unsubscribe = mapAccountTx.signAndSend(SENDER, (result) => {
      if (result.status.isFinalized) {
        unsubscribe();
        resolve();
      } else if (result.status.isInvalid) {
        unsubscribe();
        reject(new Error('MapAccount transaction is invalid'));
      }
    });
  });
}

async function executeCall() {
  const callTx = api.tx.revive.call(
    args.dest,
    args.value.toString(),
    {
      refTime: args.gasLimit.refTime.toString(),
      proofSize: args.gasLimit.proofSize.toString()
    },
    args.storageDepositLimit.toString(),
    args.data
  );

  return new Promise((resolve, reject) => {
    const unsubscribe = callTx.signAndSend(SENDER, (result) => {
      if (result.status.isFinalized) {
        const blockHash = result.status.asFinalized;
        console.log(`Smart contract call finalized in block: ${blockHash}`);
        console.log(`View execution results at: https://portal.qfnetwork.xyz/#/explorer/query/${blockHash}`);
        unsubscribe();
        resolve(blockHash);
      } else if (result.status.isInvalid) {
        unsubscribe();
        reject(new Error('Smart contract call transaction is invalid'));
      }
    });
  });
}
