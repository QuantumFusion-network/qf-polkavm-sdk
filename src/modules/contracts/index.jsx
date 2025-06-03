import {View} from "../../components/View.jsx";
import {useWallet} from "../../hooks/useAccounts.jsx";
import {useContracts} from "../../hooks/useContracts.js";
import {formatAddress} from "../../utils/utils.js";
import {Link} from "react-router-dom";
import {Button} from "../../components/button.jsx";
import {InputField} from "../../components/input.jsx";
import {useState} from "react";
import {useToast} from "../../hooks/useToast.js";

export const Contracts = () => {
  const {accounts, selected, connectWallet} = useWallet()
  const {contracts, fetchContractByAccount} = useContracts(accounts.map(({address}) => address))

  const [searchContracts, setSearchContract] = useState([])
  const [searchContractsValue, setSearchContractValue] = useState("")
  const [searchContractsLoading, setSearchContractLoading] = useState(false)


  const {onError, Toaster} = useToast()

  const onSearchContract = async (e) => {
    e.preventDefault()

    try{
      setSearchContractLoading(true)

      const contract = await fetchContractByAccount(searchContractsValue)

      setSearchContract(contract)

    }catch (e) {
      onError(e.message || "Failed search contracts")
    }

    setSearchContractLoading(false)
  }

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
      <div className="p-4">
        {!contracts.length && (
          <div className="mt-2">
            <h3 className="mt-10 font-thin text-center text-2xl">No deployed contract? <Link className="underline"
                                                                                             to="/">Deploy here</Link>
            </h3>
          </div>
        )}


        {!!contracts.length && (
          <>
            <h1 className="text-2xl">List deployed contracts {selected?.meta?.name}</h1>
            <div className="w-full">
              <div className="flex flex-wrap mt-4 mx-[-1%]">
                {contracts.map(({address, owner}) => (
                  <Link to={`/contract/${address}`} className="w-[33.3%] px-[1%] mb-2">
                    <div
                      className={`w-full cursor-pointer py-1 px-3 rounded border-[#fcda8f] border-[1px] bg-white hover:bg-[#ffc1077d]`}>
                      <span className="text-md font-bold mr-1">Address:</span>
                      <span title="copy"
                            className="text-md underline cursor-pointer">{formatAddress(address)}</span>
                      <br/>
                      <span className="text-md font-bold mr-1">Owner:</span>
                      <span title="copy"
                            className="text-md cursor-pointer">{formatAddress(owner)}</span>
                    </div>
                  </Link>
                ))}
              </div>
            </div>
          </>
        )}

        <h1 className="text-2xl mt-6">Search contracts by owner</h1>
        <form className="mt-2 mb-4" onSubmit={onSearchContract}>
          <InputField value={searchContractsValue} onChange={(e) => setSearchContractValue(e.target.value)}
                      placeholder="Owner address"/>
          <div className="w-[200px] mx-auto">
            <Button type='submit' disabled={!searchContractsValue || searchContractsLoading}
                    className="mt-2">{searchContractsLoading ? "Loading" : "Search"}</Button>
          </div>
        </form>


        {!!searchContracts.length && (
          <div className="w-full">
            <div className="flex flex-wrap mt-4 mx-[-1%]">
              {contracts.map(({address, owner}) => (
                <Link to={`/contract/${address}`} className="w-[33.3%] px-[1%] mb-2">
                  <div
                    className={`w-full cursor-pointer py-1 px-3 rounded border-[#fcda8f] border-[1px] bg-white hover:bg-[#ffc1077d]`}>
                    <span className="text-md font-bold mr-1">Address:</span>
                    <span title="copy"
                          className="text-md underline cursor-pointer">{formatAddress(address)}</span>
                    <br/>
                    <span className="text-md font-bold mr-1">Owner:</span>
                    <span title="copy"
                          className="text-md cursor-pointer">{formatAddress(owner)}</span>
                  </div>
                </Link>
              ))}
            </div>
          </div>
        )}
      </div>
    </View>
  )
}
