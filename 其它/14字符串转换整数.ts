function myAtoi(s: string) {
  if (/^(\+|\-|\d)/.test(s.trim())) {
    let num = Number(s.trim().match(/^(\-|\+)?[0-9]+(\.?[0-9]+)?/))
    return num < -(2 ** 31) ? -(2 ** 31) : num > 2 ** 31 - 1 ? 2 ** 31 - 1 : num
  }
  return 0
}

console.log(myAtoi("-42"))
