export function formatHex(integer) {
  return padInteger(integer, "0", 2).toUpperCase();
};

export function padInteger(integer, padCharacter="0", length=2) {
  var padding = new Array(length + 1).join( padCharacter );
  return (padding + integer.toString(16)).slice(-length);
};
