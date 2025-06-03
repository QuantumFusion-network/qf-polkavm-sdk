export const InputField = (props) => {
  return (
    <label>
      {!!props.textTitle && (
        <p className="mb-1">{props.textTitle}</p>
      )}
      <input className="w-full p-3 bg-white rounded border-[#0000003d] border-[1px]" {...props}/>
    </label>
  )
}
