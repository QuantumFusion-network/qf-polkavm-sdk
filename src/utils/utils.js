import {hexToU8a, isHex, u8aToHex, u8aToString} from "@polkadot/util";

export const formatAddress = (a) => `${a.slice(0, 6)}...${a.slice(a.length - 6, a.length)}`

export const BYTE_STR_0 = '0'.charCodeAt(0);
export const BYTE_STR_X = 'x'.charCodeAt(0);
export const STR_NL = '\n';
export const NOOP = () => undefined;

export function convertResult(result) {
  const data = new Uint8Array(result);

  if (data[0] === BYTE_STR_0 && data[1] === BYTE_STR_X) {
    let hex = u8aToString(data);

    while (hex.endsWith(STR_NL)) {
      hex = hex.substring(0, hex.length - 1);
    }

    if (isHex(hex)) {
      return hexToU8a(hex);
    }
  }

  return data;
}

export function extract(isCall, extrinsic, payload) {
  if (!extrinsic) {
    return ['0x', '0x', null];
  }

  const u8a = extrinsic.method.toU8a();
  let inspect = isCall
    ? extrinsic.method.inspect()
    : extrinsic.inspect();

  if (payload) {
    const prev = inspect;

    inspect = payload.inspect();
    inspect.inner?.map((entry, index) => {
      if (index === 0) {
        // replace the method inner
        entry.inner = prev.inner;
        entry.outer = undefined;
      }

      return entry;
    });
  }

  return [
    u8aToHex(u8a),
    extrinsic.registry.hash(u8a).toHex(),
    inspect
  ];
}

export function copyToClipboard(text, successCb, errorCb) {
  navigator.clipboard.writeText(text)
    .then(() => successCb())
    .catch(_err => errorCb());
}
