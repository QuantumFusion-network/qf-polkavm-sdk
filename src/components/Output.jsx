import React from 'react';

import {isString} from '@polkadot/util';


export function Output({label, isTrimmed, value,}) {
  return (
    <div className={"rounded-lg border bg-[#F5F4F4] p-3"}>
      <div className={"font-semibold text-md"}>
        {label}
      </div>
      <div className="break-words">
        {isTrimmed && isString(value) && (value.length > 512)
          ? `${value.slice(0, 256)}…${value.slice(-256)}`
          : value
        }
      </div>
    </div>
  );
}
