import {useState} from "react";
import {TypeRegistry} from '@polkadot/types';
import {Logs} from "./Logs.jsx";
import PropTypes from 'prop-types';

export const ContractMethods = ({api, injector, account, contractAddress}) => {

  const [value, setValue] = useState()
  const [logs, setLogs] = useState([])

  const onCall = async () => {
    const methodBytes = new TextEncoder().encode('main');
    const registry = new TypeRegistry();

    const argBytes = registry.createType('u32', value).toU8a();
    const tx = api.tx.qfPolkaVM.execute(
      contractAddress,
      account.address,
      BigInt(parseInt(value)),
      25,
      50000,
    );


    setLogs([])

    const _logs = []

    await tx.signAndSend(account.address, {signer: injector.signer}, ({status, events}) => {
      _logs.push("📡 Status:", status.type);

      events.forEach(({event}) => {
        const section = event.section;
        const method = event.method;
        const data = event.data.map(d => d.toHuman?.() || d.toString());

        _logs.push(`📜 ${section}.${method}`, data);

        if (section === 'system' && method === 'ExtrinsicFailed') {
          const [error] = event.data;
          if (error.isModule) {
            const decoded = api.registry.findMetaError(error.asModule);
            const {docs, name, section} = decoded;
            _logs.push(`❌ ExtrinsicFailed: ${section}.${name} - ${docs.join(' ')}`);
          } else {
            _logs.push('❌ ExtrinsicFailed (Other):', error.toString());
          }
        }
      });

      if (status.isFinalized) {
        _logs.push("🎉 Transaction finalized!");
      }

      setLogs(_logs)
    });

  }


  return (
    <div>
      <h1>Contract address: </h1>
      <div className="mb-2">{contractAddress}</div>

      <div className="mt-4 full-w rounded-lg border bg-[#F5F4F4] p-3">
        <h1 className="mb-2">Execute main: </h1>
        <input placeholder="Operation" className="w-full py-2 px-3 border rounded-lg" onChange={(e) => setValue(e.target.value)} value={value} type="text"/>
        <button
          className="mt-2 flex items-center justify-center py-2 px-3 font-karla font-semibold rounded-md transition-colors duration-200 p-3 text-[#fff] hover:bg-[#00c2489c] bg-[#00c248c9]"
          onClick={onCall}>Submit
        </button>

        <Logs className={"mt-4 max-h-[300px] overflow-auto"} label={"Logs:"} logs={logs}/>

      </div>


    </div>
  )
}


ContractMethods.propTypes = {
  api: PropTypes.object.isRequired,
  injector: PropTypes.shape({
    signer: PropTypes.object.isRequired
  }).isRequired,
  account: PropTypes.shape({
    address: PropTypes.string.isRequired,
    meta: PropTypes.object
  }).isRequired, // InjectedAccountWithMeta
  contractAddress: PropTypes.string.isRequired, // SS58 address
  contractMethods: PropTypes.arrayOf(PropTypes.string).isRequired // ['add', 'balanceOf', ...]
};
