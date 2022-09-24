/**
 * @param {string} s
 * @param {string} p
 * @return {boolean}
 */
var isMatch = function (s, p) {
  const regx = new RegExp(`${p}`)
  console.log(regx)
  return s.match(regx) ? s.match(regx)[0] === s ? true : false : false
};

console.log(isMatch('ab', 'a')); // false