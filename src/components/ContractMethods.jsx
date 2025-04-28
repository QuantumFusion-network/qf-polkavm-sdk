import {useEffect} from "react";

export const ContractMethods = ({api, contractAddress}) => {


  useEffect(() => {
    const cb = async () => {
      console.log('api ', api.query.qfPolkaVMDev?.exports)
      const exports = await api.query.qfPolkaVMDev.exports(contractAddress);


      if (exports.isSome) {
        const methods = exports.unwrap().toJSON();
        const names = methods.map((bytes) => new TextDecoder().decode(Uint8Array.from(bytes)));

        console.log("📋 Exported methods:", names);
      }
    }

    // cb()

    const rawExports = [
      '\x00\x00\x06\x01\x06\x04\x06\x04\x05\x00\x06\x00\x07\x05\x06\x00\x06\x02\x06\x05\x07\x02\x07\x03',
      '\x00\x00\x07\x03\x07\x05\x06\x02\x05\x00\x06\x00\x07\x05\x06\x00\x06\x02\x06\x05\x07\x02\x07\x03',
      '\x00\x00\x06\x00\x07\x05\x06\x00\x05\x00\x06\x00\x07\x05\x06\x00\x06\x02\x06\x05\x07\x02\x07\x03'
    ];

    const methodHexes = rawExports.map(str =>
      Array.from(str).map(c => c.charCodeAt(0).toString(16).padStart(2, '0')).join(' ')
    );

    console.log(methodHexes);


    const bytes = Uint8Array.from([
      0x00, 0x00, 0x06, 0x01, 0x06, 0x04, 0x06, 0x04,
      0x05, 0x00, 0x06, 0x00, 0x07, 0x05, 0x06, 0x00,
      0x06, 0x02, 0x06, 0x05, 0x07, 0x02, 0x07, 0x03
    ]);

    const decoded = new TextDecoder('utf-8', { fatal: true }).decode(bytes);

    console.log('decoded ', decoded);
  }, []);

  return (
    <div className="space-y-4">
      <h1>Contract address: {contractAddress}</h1>
    </div>
  )
}
