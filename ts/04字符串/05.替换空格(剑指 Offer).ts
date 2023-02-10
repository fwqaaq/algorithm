/**
 * Replace all spaces in a string with %20
 * @param {string} s - the string to replace spaces in
 * @returns The original string with the spaces replaced with "%20"
 */
function replaceSpace(s: string): string {
  let arr = s.split("")
  for (let i = arr.length - 1; i >= 0; i--) {
    if (arr[i] === " ") {
      arr[i] = "%20"
    }
  }
  return arr.join("")
}

console.log(replaceSpace("  "))
