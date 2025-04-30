export function guessType(value) {
  if (value === 'true' || value === 'false') return 'bool';
  if(isNaN(parseInt(value))) return 'string';

  const n = BigInt(value);

  if (n >= 0n) {
    if (n <= 255n) return 'u8';
    if (n <= 4294967295n) return 'u32';
    if (n <= 18446744073709551615n) return 'u64';
    if (n <= BigInt("340282366920938463463374607431768211455")) return 'u128';
  }

  return 'string';
}

export function getTypesFromArray(a) {
  return a.map((i) => guessType(i))
}
