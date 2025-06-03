import {View} from "../../components/View.jsx";
import {useWallet} from "../../hooks/useAccounts.jsx";
import {formatAddress} from "../../utils/utils.js";
import {useRpcNodes} from "../../hooks/useRpcNodes.jsx";
import {useState} from "react";
import {useToast} from "../../hooks/useToast.js";
import {Button} from "../../components/button.jsx";

export const Connect = () => {
  const {
    updateNode,
    nodes,
    selectedNode
  } = useRpcNodes()

  const {onError, onSuccess, Toaster} = useToast()
  const {connectWallet, accounts, selected, selectAddress} = useWallet({autoConnect: true})

  const onUpdate = () => {
    connectWallet()
      .then(() => onSuccess("Success update"))
      .catch(() => onError("Failed update"))
  }


  const [urlValue, setUrlValue] = useState();


  return (
    <View>
      <Toaster/>

      {!selected && (
        <div className="w-full p-4">
          <h3 className="mt-10 font-thin text-center text-2xl mb-2">Wallet is not connected!</h3>
          <Button className="max-w-[200px] mx-auto block" onClick={connectWallet}>Connect wallet</Button>
        </div>
      )}

      <div className='mt-2 px-4'>
        {!!selected && (
          <>
            <h1 className="text-2xl">Accounts</h1>
            <div className="flex flex-wrap mt-2 mx-[-1%]">
              {accounts.map((a) => (
                <div key={a.address} className="w-[33.3%] px-[1%] mb-3">
                  <div onClick={() => selectAddress(a)}
                       className={`w-full cursor-pointer py-1 px-3 rounded border-[#fcda8f] border-[1px] ${selected?.address === a?.address ? "bg-[#ffc1077d]" : "bg-white"}`}>
                    <span className="text-md font-bold mr-1">Account:</span>
                    <span title="copy" className="text-md underline cursor-pointer">{formatAddress(a.address)}</span>
                    <br/>
                    <span className="text-md font-bold mr-1">Name:</span>
                    <span title="copy" className="text-md cursor-pointer">{a?.meta?.name || '-'}</span>
                    <div className='flex items-center justify-end'>
                      <span className="mr-1 text-md">Connected</span>
                      <span className="block rounded w-[5px] h-[5px] bg-[#6fb022] mr-2"/>
                    </div>
                  </div>
                </div>
              ))}
            </div>

            <div className="mx-auto w-[150px] mt-2">
              <Button onClick={onUpdate}>
                Update
              </Button>
            </div>
          </>
        )}


      </div>

      <div className='mt-2 px-4'>
        <h1 className="text-2xl">RPC nodes</h1>

        <div className="flex flex-wrap mt-2 mx-[-1%]">
          {nodes.map((n) => (
            <div key={n.name} className="w-[33.3%] px-[1%] mb-3" onClick={() => updateNode(n)}>
              <div
                className={`w-full cursor-pointer py-1 px-3 rounded border-[#fcda8f] border-[1px] ${n.url === selectedNode.url ? "bg-[#ffc1077d]" : "bg-white"}`}>
                <span className="text-md font-bold mr-1">Url:</span>
                <span title="copy"
                      className="text-md underline cursor-pointer inline-flex max-w-[85%] overflow-hidden overflow-ellipsis whitespace-nowrap">{n.url}</span>
                <br/>
                <span className="text-md font-bold mr-1">Name:</span>
                <span title="copy" className="text-md cursor-pointer">{n.name}</span>
              </div>
            </div>
          ))}

          <div className="w-[33.3%] px-[1%] mb-3">
            <div
              className={`w-full min-h-[58px] py-1 px-3 rounded border-[#fcda8f] border-[1px] ${urlValue === selectedNode.url ? "bg-[#ffc1077d]" : "bg-white"}`}>
              <span className="text-md font-bold mr-1">Url:</span>
              <input
                placeholder="Rpc url"
                value={urlValue} onChange={(e) => setUrlValue(e.target.value)} title="copy"
                className="text-md cursor-pointer inline-flex max-w-[85%]  bg-white rounded border-[#0000003d] border-[1px] px-2"/>
              <button className="ml-1 cursor-pointer"
                      onClick={() => updateNode({name: "Custom node", url: urlValue})}>Connect
              </button>

              <br/>
              <span className="text-md font-bold mr-1">Name:</span>
              <span title="copy" className="text-md cursor-pointer">custom node</span>

            </div>
          </div>
        </div>
      </div>


    </View>
  )
}
