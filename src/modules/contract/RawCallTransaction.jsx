import {InputField} from "../../components/input.jsx";
import {Button} from "../../components/button.jsx";
import {useState} from "react";
import {useToast} from "../../hooks/useToast.js";

export const RawCallTransaction = () => {
  const [raw, setRaw] = useState("")
  const [gasLimit, setGasLimit] = useState(300_000_000_000)
  const [gasPrice, setGasPrice] = useState(10)
  const {Toaster, onError} = useToast()

  const onSubmit = (e) => {
    e.preventDefault()
    onError('Error to call contract method')
  }

  return (
    <form className="w-full mt-4" onSubmit={onSubmit}>
      <Toaster/>
      <div className="mb-3">
        <InputField requited value={raw} onChange={(e) => setRaw(e.target.value)} textTitle="Call method with raw data"
                    placeholder="raw string"/>
      </div>
      <div className="mb-3">
        <InputField type="number" requited value={gasLimit} onChange={(e) => setGasLimit(e.target.value)} textTitle="Gas limit"
                    placeholder="gas limit"/>
      </div>
      <div className="mb-3">
        <InputField type="number" requited value={gasPrice} onChange={(e) => setGasPrice(e.target.value)} textTitle="Gas price"
                    placeholder="gas price"/>
      </div>

      <Button type='submit' disabled={!raw} className="mt-4 max-w-[200px] mx-auto block">
        Call
      </Button>
    </form>
  )
}
