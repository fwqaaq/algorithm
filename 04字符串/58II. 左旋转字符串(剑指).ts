function reverseLeftWords(s: string, n: number): string {
  let arr = s.split("")

  revserString(0, n - 1, arr)
  revserString(n, s.length - 1, arr)
  revserString(0, s.length - 1, arr)
  return arr.join("")
}
/**
 * Given an array of strings, reverse the order of the strings in the array
 * @param {number} left - the starting index of the array
 * @param {number} right - the rightmost index of the array
 * @param {string[]} arr - the array to be reversed
 */
function revserString(left: number, right: number, arr: string[]) {
  let temp: string
  while (left < right) {
    temp = arr[left]
    arr[left] = arr[right]
    arr[right] = temp
    left++
    right--
  }
}

console.log(reverseLeftWords("abcdefg", 2))
