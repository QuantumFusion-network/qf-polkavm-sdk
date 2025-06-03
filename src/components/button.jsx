export const Button = (props) => (
  <button
    {...props}
    className={`w-full py-2 bg-[#4caf50] text-white font-karla font-semibold rounded-md hover:bg-[#4caf50b0] disabled:bg-[#795548] transition-colors duration-200 disabled:opacity-50 ${props.className}`}>
  </button>
)
