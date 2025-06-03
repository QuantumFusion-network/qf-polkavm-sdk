import {View} from "../../components/View.jsx";
import {useApi} from "../../hooks/useApi.jsx";
import {convertResult, extract, NOOP} from "../../utils/utils.js";
import {useWallet} from "../../hooks/useAccounts.jsx";
import {web3FromAddress} from "@polkadot/extension-dapp";
import {useState} from "react";
import {Output} from "../../components/Output.jsx";
import {DecodedInspect} from "../../components/DecodedInspect.jsx";
import {Logs} from "../../components/Logs.jsx";
import {useContracts} from "../../hooks/useContracts.js";
import {useToast} from "../../hooks/useToast.js";
import {Button} from "../../components/button.jsx";

export const Deploy = () => {
  const {api, error, isReady} = useApi()
  const {selected, accounts, connectWallet} = useWallet()
  const {saveContract} = useContracts(accounts.map(({address}) => address))
  const {onError, onSuccess, Toaster} = useToast()
  const [deployLogs, setDeployLogs] = useState([])
  const [{extrinsic, info}, setExtrinsic] = useState({
    extrinsic: null,
    info: null
  })

  if(error) return  error

  if (!isReady) {
    return (
      <View>
        <h1 className='mt-2 px-4'>Connecting</h1>
      </View>
    )
  }


  const onUpload = async (e) => {
    const file = e.target.files?.[0];

    try {
      const reader = new FileReader();

      reader.onabort = NOOP;
      reader.onerror = NOOP;

      reader.onload = ({target}) => {
        if (target?.result) {
          const data = convertResult(target.result);

          const hex = Array.from(data)
            .map(b => b.toString(16).padStart(2, '0'))
            .join('');

          const extrinsic = api.tx.qfPolkaVM.upload("0x" + hex);

          const [
            _hex,
            registryHash,
            inspect
          ] = extract(true, extrinsic, undefined)

          setExtrinsic({
            extrinsic,
            info: {
              hex: _hex,
              registryHash,
              inspect,
              name: file.name
            }
          })
        }
      };

      reader.readAsArrayBuffer(file);

      onSuccess("File uploaded")

    } catch (error) {
      onError("Filed uploaded")

      console.error("Upload error:", error);
      throw error;
    }

  }

  const onDeploy = async () => {
    const injector = await web3FromAddress(selected.address);

    if (!extrinsic || !injector) throw "extrinsic or injector ERROR"

    setDeployLogs([])

    await extrinsic.signAndSend(selected.address, {signer: injector.signer}, ({status, events}) => {
      const _logs = [];
      _logs.push("📡 Status update:", status.type);

      if (status.isInBlock) _logs.push("📦 Included in block:", status.asInBlock.toHex());
      if (status.isFinalized) _logs.push("🎉 Finalized in block:", status.asFinalized.toHex());

      if (events && events.length > 0) {
        _logs.push("📜 Events received:");
        events.forEach(({phase, event: {section, method, data}}, index) => {
          _logs.push(`  [${index}] ${section}.${method} (phase: ${phase.toString()})`);
          data.forEach((arg, i) => {
            _logs.push(`    └─ Arg[${i}]:`, arg.toHuman());
          });
        });
      } else {
        _logs.push("⚠️ No events received.");
      }

      const success = events.find(({event}) => event.section === 'system' && event.method === 'ExtrinsicSuccess');
      if (success) _logs.push("✅ Extrinsic executed successfully.");

      const failure = events.find(({event}) => event.section === 'system' && event.method === 'ExtrinsicFailed');
      if (failure) {
        const dispatchError = failure.event.data[0];
        if (dispatchError.isModule) {
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          _logs.push(`❌ Extrinsic failed: ${decoded.section}.${decoded.name}`);
          _logs.push(`📖 Reason: ${decoded.docs.join(' ')}`);
        } else {
          _logs.push("❌ Extrinsic failed with error:", dispatchError.toString());
          notify({message: "Extrinsic failed", type: 'error'})
        }
      }

      const uploadedEvent = events.find(
        ({event}) => event.section === "qfPolkaVM" && event.method === "ProgramBlobUploaded"
      );

      if (uploadedEvent) {
        const contractAddress = uploadedEvent.event.data[1].toString();
        _logs.push("📦 Contract uploaded at:", contractAddress);
        saveContract(contractAddress, selected.address)

        // const exportsRaw = uploadedEvent.event.data[2];
        // const methodNames = exportsRaw.map(bytes => new TextDecoder('utf-8').decode(bytes));

        // if(methodNames.length) setContractMethods(methodNames)

      } else {
        console.warn("⚠️ qfPolkaVM.ProgramBlobUploaded event not found.");
        _logs.push("⚠️ qfPolkaVM.ProgramBlobUploaded event not found.");
      }

      setDeployLogs(_logs)
    });
  }

  const onCancel = () => setExtrinsic({
    extrinsic: null,
    info: null
  })

  if(!selected) {
    return (
      <View>
        <div className="w-full p-4">
          <h3 className="mt-10 font-thin text-center text-2xl mb-2">Wallet is not connected!</h3>
          <Button className="max-w-[200px] mx-auto block" onClick={connectWallet}>Connect wallet</Button>
        </div>
      </View>
    )
  }

  return (
    <View>
      <Toaster/>
      <div className="w-full p-4">
        <h1 className="text-2xl mb-2">Deploy contract</h1>

        {!extrinsic && !info?.hex && (
          <label className="w-full h-[100px] bg-white flex rounded border-[#fcda8f] border-[1px] cursor-pointer">
            <h1 className="m-auto">No file chosen</h1>
            <input className="w-0  h-0 opacity-0" type="file" accept=".polkavm" onChange={onUpload}/>
          </label>
        )}

        {!!extrinsic && (
          <>
            <div
              className="p-3 w-full min-h-[200px] bg-white mt-2 rounded border-[#fcda8f] border-[1px]">

              <h3 className="mb-3">Predeploy info</h3>

              <div className="mb-3">
                <Output label={"File name"} isTrimmed value={info.name}/>
              </div>
              <div className="mb-3">
                <Output label={"encoded call data"} isTrimmed value={info.hex}/>
              </div>
              <div className="mb-3">
                <Output label={"encoded call hash"} value={info.registryHash}/>
              </div>
              <div className="mb-3">
                <DecodedInspect hex={info.hex} inspect={info.inspect}/>
              </div>

              <div className="flex justify-between">
                <Button disabled={!api.isReady || !extrinsic} onClick={onDeploy} className="w-[49.5%] mr-1">
                  Deploy
                </Button>
                <Button onClick={onCancel} className="w-[49.5%] bg-amber-800 hover:bg-amber-600">
                  Cancel
                </Button>
              </div>
            </div>

            {!!deployLogs.length && (
              <div className="mt-4 p-3 w-full min-h-[200px] bg-white rounded border-[#fcda8f] border-[1px]">
                <Logs className={"max-h-[300px] overflow-auto"} label={"Deploy logs:"} logs={deployLogs}/>
              </div>
            )}
          </>
        )}

      </div>
    </View>
  )
}
