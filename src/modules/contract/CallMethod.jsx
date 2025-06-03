import {InputField} from "../../components/input.jsx";
import {Button} from "../../components/button.jsx";
import {useRef, useState} from "react";
import {useOutsideClick} from "../../hooks/useOutsideClick.jsx";
import {useToast} from "../../hooks/useToast.js";

export const CallMethod = () => {
  const [isOpen, setOpen] = useState(false)
  const [selected, setSelected] = useState(null)
  const [gasLimit, setGasLimit] = useState(300_000_000_000)
  const [gasPrice, setGasPrice] = useState(10)
  const onToggle = () => setOpen(!isOpen)
  const {Toaster, onError} = useToast()
  const triangle = '▼';

  const ref = useRef();

  const options = [
    {label: 'Transfer', value: '0x00'},
    {label: 'Balance', value: '0x01'},
    {label: 'BalanceOf', value: '0x02'},
    {label: 'BlockNumber', value: '0x03'},
    {label: 'InfinityLoop', value: '0x04'},
    {label: 'Inc', value: '0x05'},
    {label: 'Delete', Delete: '0x06'},
  ]

  useOutsideClick(ref, () => {
    setOpen(false);
  });


  const onSelect = ({label, value}) => () =>  {
    setSelected({label, value})
    setOpen(false)
  }

  const onSubmit = (e) => {
    e.preventDefault()
    onError('Error to call contract method')
  }

  return (
    <form className="w-full mt-4" onSubmit={onSubmit}>
      <Toaster/>
      <div className="mb-3 relative" ref={ref}>
        <p>Select method</p>
        <div onClick={onToggle}
             className="w-full p-3 bg-white rounded border-[#0000003d] border-[1px] cursor-pointer flex justify-between">
          <span>{selected?.label || '–'}</span>
          <span className="select-none" style={{transform: `rotate(${isOpen ? 180 : 0}deg)`}}>{triangle}</span>
        </div>

        {isOpen && (
          <div className="z-20 absolute top-[80px] w-full p-1 bg-white rounded border-[#0000003d] border-[1px]">
            {options.map(({label, value}) => (
              <div onClick={onSelect({label, value})} className="hover:bg-[#dfdddd] p-1 px-2 rounded cursor-pointer">
                {label}
              </div>
            ))}
          </div>
        )}
      </div>

      <div className="mb-3">
        <InputField type="number" requited value={gasLimit} onChange={(e) => setGasLimit(e.target.value)}
                    textTitle="Gas limit"
                    placeholder="gas limit"/>
      </div>
      <div className="mb-3">
        <InputField type="number" requited value={gasPrice} onChange={(e) => setGasPrice(e.target.value)}
                    textTitle="Gas price"
                    placeholder="gas price"/>
      </div>


      <Button disabled={!selected} type='submit' className="relative z-10 mt-4 max-w-[200px] mx-auto block">
        Call
      </Button>
    </form>
  )
}
