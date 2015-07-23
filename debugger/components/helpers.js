export function formatBinary(integer) {
  return padInteger(integer, "0", 8, 2).toUpperCase();
};

export function formatHex(integer) {
  return padInteger(integer, "0", 2, 16).toUpperCase();
};

export function padInteger(integer, padCharacter="0", length=2, base=16) {
  var padding = new Array(length + 1).join( padCharacter );
  if (typeof integer !== 'undefined') {
    return (padding + integer.toString(base)).slice(-length);
  }
  else {
    return padding;
  }
};
