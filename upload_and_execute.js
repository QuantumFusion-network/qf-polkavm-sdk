// Script to call the 'upload' and 'execute' extrinsics from the qfPolkaVm pallet
// and handle events

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
const value = 0;
const userData = '0x00';
const gasLimit = 100;
const gasPrice = 10;

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
    // Track the transaction status
    if (status.isInBlock) {
      console.log(`Transaction included in block with hash: ${status.asInBlock.toHex()}`);
    } else if (status.isFinalized) {
      console.log(`Transaction finalized in block with hash: ${status.asFinalized.toHex()}`);
    } else {
      console.log(`Transaction status: ${status.type}`);
    }

    // Handle events
    events.forEach(({ phase, event: { data, method, section } }) => {
      // console.log(`${phase.toString()} : ${section}.${method} ${data.toString()}`);
      
      // Specifically handle the ProgramBlobUploaded event
      if (section === 'qfPolkaVM' && method === 'ProgramBlobUploaded') {
        const contractAddress = data[1].toString();
        console.log(`\nProgram uploaded successfully!`);
        console.log(`Contract address: ${contractAddress}`);
        
        // Additional logic for working with the contract can be added here
        unsubscribeUploadExtrinsic();
        resolve(contractAddress);
      }
    });
    
    // Check for errors
    const errorEvent = events.find(({ event }) => 
      api.events.system.ExtrinsicFailed.is(event)
    );
    
    if (errorEvent) {
      // Extract error information
      const { event: { data: [error] } } = errorEvent;
      
      if (error.isModule) {
        // For module errors, decode it
        const decoded = api.registry.findMetaError(error.asModule);
        const { docs, method, section } = decoded;
        
        console.log(`\nError: ${section}.${method}: ${docs.join(' ')}`);
      } else {
        // Other errors
        console.log(`\nError: ${error.toString()}`);
      }
    }
  });
});

console.log('Transaction sent. Waiting for processing...');

const contractAddress = await contractAddressPromise;

const executeExtrinsic = api.tx.qfPolkaVM.execute(contractAddress, to, value, userData, gasLimit, gasPrice);
console.log(`\nExtrinsic created: ${executeExtrinsic.method.section}.${executeExtrinsic.method.method}`);

const unsubscribeExecuteExtrinsic = await executeExtrinsic.signAndSend(ALICE, ({ events = [], status }) => {
  // Track transaction status
  if (status.isInBlock) {
    console.log(`Transaction included in block with hash: ${status.asInBlock.toHex()}`);
  } else if (status.isFinalized) {
    console.log(`Transaction finalized in block with hash: ${status.asFinalized.toHex()}`);
  } else {
    console.log(`Transaction status: ${status.type}`);
  }

  // Handle events
  events.forEach(({ phase, event: { data, method, section } }) => {
    // console.log(`${phase.toString()} : ${section}.${method} ${data.toString()}`);

    if (section === 'qfPolkaVM' && method === 'ExecutionResult') {
      const [_who, _contractAddress, result, notEnoughGas, trap, gasBefore, gasAfter] = data;
      console.log(`\nProgram executed successfully!`);
      
      console.log(JSON.stringify({
        result: api.createType('Option<u64>', result).toHuman(),
        notEnoughGas,
        trap,
        gasBefore: api.createType('u32', gasBefore).toHuman(),
        gasAfter: api.createType('i64', gasAfter).toHuman(),
      }, null, 2));
      
      // Additional logic for working with the contract can be added here
      unsubscribeExecuteExtrinsic();
    }
  });
  
  // Check for errors
  const errorEvent = events.find(({ event }) => 
    api.events.system.ExtrinsicFailed.is(event)
  );
  
  if (errorEvent) {
    // Extract error information
    const { event: { data: [error] } } = errorEvent;
    
    if (error.isModule) {
      // For module errors, decode it
      const decoded = api.registry.findMetaError(error.asModule);
      const { docs, method, section } = decoded;
      
      console.log(`\nError: ${section}.${method}: ${docs.join(' ')}`);
    } else {
      // Other errors
      console.log(`\nError: ${error.toString()}`);
    }
  }
});

console.log('Transaction sent. Waiting for processing...');
