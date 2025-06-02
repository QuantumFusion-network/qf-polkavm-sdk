// Script to call the 'upload' and 'execute' extrinsics from the qfPolkaVm pallet
// and handle events. You could paste that script to https://portal.qfnetwork.xyz/#/js

// Use ALICE as the transaction sender
const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

// Prepare program data for upload
// 'get-account-balance.polkavm'
// python3 -c "print(', '.join(f'0x{b:02X}' for b in open('output/get-account-balance.polkavm', 'rb').read()))"
const programData = new Uint8Array([
  0x50, 0x56, 0x4D, 0x00, 0x01, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x04, 0x00, 0x00, 0xA0, 0x00, 0x04, 0x0F, 0x01, 0x00, 0x00, 0x00, 0x00, 0x62, 0x61, 0x6C, 0x61, 0x6E, 0x63, 0x65, 0x5F, 0x6F, 0x66, 0x05, 0x07, 0x01, 0x00, 0x04, 0x6D, 0x61, 0x69, 0x6E, 0x06, 0x12, 0x00, 0x00, 0x0D, 0x83, 0x11, 0xFC, 0x7A, 0x10, 0x0A, 0x81, 0x10, 0x83, 0x11, 0x04, 0x32, 0x00, 0x69, 0x09, 0x00
]);

// Extrinsic execute parameters
const to = ALICE;
const value = 10;
const userData = '0x00';
const gasLimit = 10000;
const gasPrice = 10;

const contractFundAmount = '1000000000000000000';

console.log(`Prepared program blob of size ${programData.length} bytes`);

function buf2hex(buffer) { // buffer is an ArrayBuffer
  return [...new Uint8Array(buffer)]
    .map(x => x.toString(16).padStart(2, '0'))
    .join('');
}

// Create the 'upload' extrinsic from the qfPolkaVm pallet with the programBlob argument
const uploadExtrinsic = api.tx.qfPolkaVM.upload('0x' + buf2hex(programData));
console.log(`Extrinsic created: ${uploadExtrinsic.method.section}.${uploadExtrinsic.method.method}`);

// Sign and send the transaction
const contractAddressPromise = new Promise(async (resolve) => {
  const unsubscribeUploadExtrinsic = await uploadExtrinsic.signAndSend(ALICE, ({ events = [], status }) => {
    // Handle events
    events.forEach(({ phase, event: { data, method, section } }) => {
      // Specifically handle the ProgramBlobUploaded event
      if (section === 'qfPolkaVM' && method === 'ProgramBlobUploaded') {
        const contractAddress = data[1].toString();

        unsubscribeUploadExtrinsic();
        resolve(contractAddress);
      }
    });
  });
});
console.log('Transaction sent. Waiting for processing...');
const contractAddress = await contractAddressPromise;
console.log('Program uploaded successfully!');
console.log(`Contract address: ${contractAddress}`);

// Fund smart contract account
const transfer = api.tx.balances.transferAllowDeath(contractAddress, contractFundAmount);
const transferPromise = new Promise(async (resolve) => {
  const unsubscribeTransfer = await transfer.signAndSend(ALICE, ({ events = [], status }) => {
    events.forEach(({ phase, event: { data, method, section } }) => {
      if (section === 'balances' && method === 'Transfer') {
        unsubscribeTransfer();
        resolve();
      }
    });
  });
});
await transferPromise;

const executeExtrinsic = api.tx.qfPolkaVM.execute(contractAddress, to, value, userData, gasLimit, gasPrice);
console.log(`Extrinsic created: ${executeExtrinsic.method.section}.${executeExtrinsic.method.method}`);

const executionDataPromise = new Promise(async (resolve) => {
  const unsubscribeExecuteExtrinsic = await executeExtrinsic.signAndSend(ALICE, ({ events = [], status }) => {
    events.forEach(({ phase, event: { data, method, section } }) => {
      if (section === 'qfPolkaVM' && method === 'ExecutionResult') {
        unsubscribeExecuteExtrinsic();
        resolve(data);
      }
    });
  });
});
console.log('Transaction sent. Waiting for processing...');
const executionData = await executionDataPromise;

const [_who, _contractAddress, version, result, notEnoughGas, trap, gasBefore, gasAfter] = executionData;
console.log('Program executed successfully!');

console.log(JSON.stringify({
  version: api.createType('u64', version).toHuman(),
  result: api.createType('Option<u64>', result).toHuman(),
  notEnoughGas,
  trap,
  gasBefore: api.createType('u32', gasBefore).toHuman(),
  gasAfter: api.createType('i64', gasAfter).toHuman(),
}, null, 2));

// Second call
(async () => {
  const executeExtrinsic = api.tx.qfPolkaVM.execute(contractAddress, to, value, userData, gasLimit, gasPrice);
  console.log(`Extrinsic created: ${executeExtrinsic.method.section}.${executeExtrinsic.method.method}`);

  const executionDataPromise = new Promise(async (resolve) => {
    const unsubscribeExecuteExtrinsic = await executeExtrinsic.signAndSend(ALICE, ({ events = [], status }) => {
      events.forEach(({ phase, event: { data, method, section } }) => {
        if (section === 'qfPolkaVM' && method === 'ExecutionResult') {
          unsubscribeExecuteExtrinsic();
          resolve(data);
        }
      });
    });
  });
  console.log('Transaction sent. Waiting for processing...');
  const executionData = await executionDataPromise;

  const [_who, _contractAddress, version, result, notEnoughGas, trap, gasBefore, gasAfter] = executionData;
  console.log('Program executed successfully!');

  console.log(JSON.stringify({
    version: api.createType('u64', version).toHuman(),
    result: api.createType('Option<u64>', result).toHuman(),
    notEnoughGas,
    trap,
    gasBefore: api.createType('u32', gasBefore).toHuman(),
    gasAfter: api.createType('i64', gasAfter).toHuman(),
  }, null, 2));

})()
