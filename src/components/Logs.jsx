import React from 'react';

export function Logs({logs, label, className}) {

  if (!logs?.length) {
    return
  }

  return (
    <div>
      {!!label && (
        <h1 className="mb-1">{label}</h1>
      )}
      <div className={"full-w rounded-lg border bg-[#F5F4F4] p-3 ".concat(className || "")}>

        {logs.map((l, i) => (
          <div className={"break-words max-w-[100%] mb-2"} key={i}>
            {typeof l === 'string' ? l : JSON.stringify(l)}
          </div>
        ))}
      </div>
    </div>
  );
}
