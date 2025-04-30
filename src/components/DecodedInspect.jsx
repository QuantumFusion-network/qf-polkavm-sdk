import {u8aToHex} from "@polkadot/util";
import {useMemo} from "react";
import PropTypes from "prop-types";

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

export const DecodedInspect = ({inspect}) => {
  const formatted = useMemo(
    () => inspect && formatInspect(inspect),
    [inspect]
  );

  if (!formatted) {
    return null;
  }

  return (
    <div className={"rounded-lg border bg-[#F5F4F4] p-3"}>
      {formatted.map(({name, value}, i) => (
        <div className="flex flex-wrap" key={i}>
          {!!name && (
            <div><label className={"font-semibold text-md pr-2"}>{name}:</label></div>
          )}

          <div className="max-w-[100%] break-words">
            {value.trim()}
          </div>
        </div>
      ))}

    </div>
  )
}

DecodedInspect.propTypes = {
  inspect: PropTypes.func.isRequired
};
