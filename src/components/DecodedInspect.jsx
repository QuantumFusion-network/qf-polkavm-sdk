import {u8aToHex} from "@polkadot/util";
import {useMemo} from "react";

function formatInspect({inner = [], name = '', outer = []}, result = []) {
  if (outer.length) {
    const value = new Array(outer.length);

    for (let i = 0; i < outer.length; i++) {
      value[i] = u8aToHex(outer[i], undefined, false);
    }

    result.push({name, value: value.join(' ')});
  }

  for (let i = 0, count = inner.length; i < count; i++) {
    formatInspect(inner[i], result);
  }

  return result;
}

export const DecodedInspect = ({inspect, hex}) => {
  const formatted = useMemo(
    () => inspect && formatInspect(inspect),
    [inspect]
  );

  if (!formatted) {
    return null;
  }

  return (
    <div className={"rounded-lg border bg-[#F5F4F4] p-3"}>
      <table>
        <tbody>
        {formatted.map(({name, value}, i) => (
          <tr key={i}>
            <td><label className={"font-semibold text-md pr-2"}>{name}</label></td>
            <td>{value}</td>
          </tr>
        ))}
        </tbody>
      </table>

    </div>
  )

}
