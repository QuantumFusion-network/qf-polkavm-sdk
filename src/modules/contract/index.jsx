import {View} from "../../components/View.jsx";
import {useParams} from "react-router-dom";
import {Button} from "../../components/button.jsx";
import {useWallet} from "../../hooks/useAccounts.jsx";

export const Contract = () => {

  const {contractAddress} = useParams()
  const {selected, connectWallet} = useWallet()

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
      <div className="p-4">
        <h1 className="text-2xl">Contract: {contractAddress}</h1>

        <div className="mt-2">
          <label>
            <p className="mb-1">Call method</p>
            <input className="w-full p-3 bg-white rounded border-[#0000003d] border-[1px]" placeholder="router, params"/>
          </label>

          <button
            className="block mx-auto mt-3 max-w-[300px] w-full box-border px-1 py-2.5 bg-[#777777] text-white font-karla font-semibold rounded-md hover:bg-[#676767] transition-colors duration-200 disabled:opacity-50">
            Submit
          </button>
        </div>
      </div>
    </View>
  )
}
