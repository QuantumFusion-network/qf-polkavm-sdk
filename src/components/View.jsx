import {useApi} from "../hooks/useApi.jsx";
import {useWallet} from "../hooks/useAccounts.jsx";
import {copyToClipboard, formatAddress} from "../utils/utils.js";
import {Link} from "react-router-dom";
import {useToast} from "../hooks/useToast.js";
import {Button} from "./button.jsx";

export const View = ({children}) => {
  const api = useApi()
  const {connectWallet, selected, disconnectWallet} = useWallet({api, autoConnect: true})

  const {onError, onSuccess, Toaster} = useToast()

  const onCopy = () => (
    void copyToClipboard(selected.address,
      () => onSuccess("Address copied to clipboard!"),
      () => onError("Failed to copy!"),
    )
  )

  return (
    <main className="w-full bg-[#f5f4f4] min-h-screen relative">
      <Toaster/>
      <img src="/circle.webp" className="absolute left-0 top-[10%] z-0"/>
      <header className="w-full py-2 px-4 max-w-[998px] mx-auto flex justify-between items-center z-10">
        <nav className='flex w-min'>
          <Link className="text-md py-0.5 px-1.5 mr-2 bg-[#ffc1077d] hover:bg-[#ffc107] rounded" to="/">Deploy</Link>
          <Link className="text-md py-0.5 px-1.5 mr-2 bg-[#ffc1077d] hover:bg-[#ffc107] rounded"
                to="/contracts">Contracts</Link>
          <Link className="text-md py-0.5 px-1.5 mr-2 bg-[#ffc1077d] hover:bg-[#ffc107] rounded"
                to="/connect">Connect</Link>
        </nav>

        {selected ? (
          <div className={"flex"}>
            <div onClick={onCopy}
                 className="px-2 py-2 bg-white rounded border-[#0000003d] border-[1px] cursor-pointer z-10 mr-2">
              {formatAddress(selected.address)}
            </div>
            <Button onClick={disconnectWallet} className="px-2">
              Disconnect
            </Button>
          </div>
        ) : (
          <button className="z-10" onClick={connectWallet}>
            Connect wallet
          </button>
        )}


      </header>

      <div className="z-10 border-b-[1px] border-[#ffc1077d] w-full mx-auto max-w-[965px]"/>

      <section className="z-10 relative max-w-[998px] mx-auto">
        {children}
      </section>
    </main>
  )
}
