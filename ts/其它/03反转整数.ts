/**
 * @description 整数的反转
 * @author jack-z
 * @date 06/10/2021
 * @param {number} x
 * @returns {*}  {number}
 */
function reverse(x: number): number {
  let a = String(Math.abs(x));
  let array: string[] = [];
  for (let i = 0; i < a.length; i++) {
    array.unshift(a[i]);
  }
  let s: string = "";
  for (let value of array) {
    s += value;
  }
  let value = Number(s);
  if (x < 0) {
    return -value < 0 - 2 ** 31 ? 0 : -value;
  } else {
    return value > 2 ** 31 - 1 ? 0 : value;
  }
}

console.log(reverse(-2147483648));
