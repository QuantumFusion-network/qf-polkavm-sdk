import {Output} from "./Output.jsx";
import {DecodedInspect} from "./DecodedInspect.jsx";
import {Logs} from "./Logs.jsx";
import {useState} from "react";
import {hexToU8a, isHex, u8aToHex, u8aToString} from "@polkadot/util";

const BYTE_STR_0 = '0'.charCodeAt(0);
const BYTE_STR_X = 'x'.charCodeAt(0);
const STR_NL = '\n';
const NOOP = () => undefined;

function convertResult(result) {
  const data = new Uint8Array(result);

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

export const ContractStep = ({api, account, injector, setContractAddress, setContractMethods}) => {

  const [deployLogs, setDeployLogs] = useState([]);
  const [extrinsic, setExtrinsic] = useState();
  const [info, setInfo] = useState();

  const onUpload = async (e) => {
    const file = e.target.files?.[0];
    if (file && api && account && injector) {
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

  const onSend = async () => {
    if (!extrinsic) {
      return
    }
    setDeployLogs([])

    await extrinsic.signAndSend(account.address, {signer: injector.signer}, ({status, events}) => {
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
        }
      }

      const uploadedEvent = events.find(
        ({event}) => event.section === "qfPolkaVM" && event.method === "ProgramBlobUploaded"
      );

      if (uploadedEvent) {
        const contractAddress = uploadedEvent.event.data[1].toString();
        _logs.push("📦 Contract uploaded at:", contractAddress);
        setContractAddress(contractAddress)

        const exportsRaw = uploadedEvent.event.data[2];
        const methodNames = exportsRaw.map(bytes => new TextDecoder('utf-8').decode(bytes));

        if(methodNames.length) setContractMethods(methodNames)

      } else {
        console.warn("⚠️ qfPolkaVM.ProgramBlobUploaded event not found.");
        _logs.push("⚠️ qfPolkaVM.ProgramBlobUploaded event not found.");
      }

      setDeployLogs(_logs)
    });
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
          onClick={onSend}
          className="w-full flex items-center justify-center gap-4 px-2 py-3 font-karla font-semibold rounded-md transition-colors duration-200 p-3 text-[#fff] hover:bg-[#00c2489c] bg-[#00c248c9]  rounded-md flex items-center gap-2">Submit</button>
      )}

      <Logs className={"max-h-[300px] overflow-auto"} label={"Deploy logs:"} logs={deployLogs}/>
    </div>
  )
}
