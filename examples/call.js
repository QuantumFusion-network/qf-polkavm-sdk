/*
 * Smart contract call script for QuantumFusion network.
 * Paste this into https://portal.qfnetwork.xyz/#/js.
 */

// Configuration
const SENDER = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'; // an account connected from the wallet for signing the transaction
const CONTRACT_ADDRESS = '0x0000000000000000000000000000000000000000'; // get it at https://portal.qfnetwork.xyz/#/chainstate calling `revive` pallet `contractInfoOf` getter
const CALL_DATA = '0xffffff00'; // SCALE encoded unsigned 32-bit integer 16777215, see https://docs.polkadot.com/polkadot-protocol/parachain-basics/data-encoding/#data-types

const DECIMALS = 10n ** 18n;
const WEIGHT = 10n ** 9n;

// Main execution
console.log('Setting up account mapping...');
await ensureAccountMapped();

console.log('Executing smart contract call...');
const blockHash = await callContract();

console.log(`Contract call completed in block: ${blockHash}`);
console.log(`View results: https://portal.qfnetwork.xyz/#/explorer/query/${blockHash}`);

// Helper functions
async function ensureAccountMapped() {
  const mappedAddress = await findMappedAccount();

  if (mappedAddress) {
    console.log(`Account already mapped to: ${mappedAddress}`);
    return;
  }

  console.log('Mapping account...');
  await mapAccount();

  const newAddress = await findMappedAccount();
  if (newAddress) {
    console.log(`Account mapped to: ${newAddress}`);
  } else {
    throw new Error('Account mapping failed');
  }
}

async function findMappedAccount() {
  const entries = await api.query.revive.originalAccount.entries();

  for (const [key, value] of entries) {
    if (value.toString() === SENDER) {
      const addressBytes = key.args[0].toU8a();
      return `0x${Buffer.from(addressBytes).toString('hex')}`;
    }
  }
  return null;
}

async function mapAccount() {
  const tx = api.tx.revive.mapAccount();
  return signAndWait(tx);
}

async function callContract() {
  const tx = api.tx.revive.call(
    CONTRACT_ADDRESS,
    DECIMALS.toString(),
    {
      refTime: WEIGHT.toString(),
      proofSize: WEIGHT.toString()
    },
    DECIMALS.toString(),
    CALL_DATA
  );
  return signAndWait(tx);
}

async function signAndWait(tx) {
  return new Promise((resolve, reject) => {
    tx.signAndSend(SENDER, (result) => {
      if (result.status.isFinalized) {
        resolve(result.status.asFinalized);
      } else if (result.status.isInvalid) {
        reject(new Error('Transaction is invalid'));
      }
    });
  });
}
