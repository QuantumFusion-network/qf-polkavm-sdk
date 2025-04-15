import React from 'react';

export function Logs({logs}) {

  if(!logs?.length) {
    return
  }

  return (
    <div className={"full-w rounded-lg border bg-[#F5F4F4] p-3"}>
      {logs.map((l) => (
        <div key={l}>
          {l}
        </div>
      ))}
    </div>
  );
}
