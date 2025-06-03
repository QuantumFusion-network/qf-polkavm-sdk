import {useEffect, useState} from "react";
import {BN} from "@polkadot/util";
import {useApi} from "./useApi.jsx";

export const useContracts = (wallets) => {
  const [contracts, setContracts] = useState([])
  const {api, isReady} = useApi()

  async function fetchContractByAccount(account) {
    if(!isReady) return

    let contracts = [];
    let currentTask = true;
    let counter = 1;

    while(currentTask) {
      const info = await api.query.qfPolkaVM.codeAddress([account, new BN(counter)]);

      if (info?.isSome) {
        contracts.push({
          owner: account,
          address: info.value.toString()
        })

        counter = counter +1
      } else {
        currentTask = false
      }
    }

    return contracts
  }

  const fetchContracts = async (_wallets) => {
   const contracts = await Promise.all(_wallets.map(async (w) => await fetchContractByAccount(w)))

    const flattedContracts = contracts.reduce((acc, c) => [...acc, ...c],[])

    setContracts(flattedContracts)
  }

  useEffect(() => {
    void fetchContracts(wallets)
  }, [...wallets, api?.isReady]);


  return {
    contracts,
    refetchContracts: () => fetchContracts(wallets),
    fetchContractByAccount
  }
}
