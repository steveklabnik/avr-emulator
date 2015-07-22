export function formatHex(integer) {
  return ("00" + integer.toString(16)).slice(-2).toUpperCase();
};
