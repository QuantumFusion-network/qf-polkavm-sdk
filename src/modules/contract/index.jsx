import {View} from "../../components/View.jsx";
import {useParams} from "react-router-dom";
import {Button} from "../../components/button.jsx";
import {useWallet} from "../../hooks/useAccounts.jsx";
import {RawCallTransaction} from "./RawCallTransaction.jsx";
import {CallMethod} from "./CallMethod.jsx";
import {useState} from "react";

export const Contract = () => {

  const {contractAddress} = useParams()
  const {selected, connectWallet} = useWallet()
  const [active, setActive] = useState(0)

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

        <div className="flex items-center justify-between w-[300px] relative mx-auto mt-3">
          <span onClick={() => setActive(0)}
                className={`cursor-pointer w-[50%] text-center text-md py-0.5 px-1.5 mr-2 bg-[${active === 0 ? '#ffc1077d' : '#f5f4f4'}] hover:bg-[#ffc107] rounded`}>Call method</span>
          <span onClick={() => setActive(1)}
                className={`cursor-pointer w-[50%] text-center text-md py-0.5 px-1.5 mr-2 bg-[${active === 1 ? '#ffc1077d' : '#f5f4f4'}] hover:bg-[#ffc107] rounded`}>Raw data</span>

        </div>
        {active === 0 && (
          <CallMethod/>
        )}

        {active === 1 && (
          <RawCallTransaction/>
        )}



      </div>
    </View>
  )
}
