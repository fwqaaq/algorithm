function minWindow(s: string, t: string) {
  let tArr = t.split("").sort()
  let index: number[] = []
  let min = 0
  let max = 0
  let word = ""
  let subStr = s.split("")
  if (t.length > s.length) return ""

  while (
    tArr.every((item) => {
      if (subStr.indexOf(item) !== -1) {
        index.push(subStr.indexOf(item))
        subStr[subStr.indexOf(item)] = ""
        return true
      }
    }) &&
    s.length >= t.length
  ) {
    min = Math.min(...index)
    max = Math.max(...index)
    index = []
    let sub = s.substring(min, max + 1)
    word =
      word === ""
        ? sub
        : word.length > sub.length && sub.length >= tArr.length
        ? sub
        : word
    s = s.slice(min + 1)
    subStr = [...s.split("")]
  }
  return word
}

console.log(minWindow("caacabbaccabbbc", "abbcaaabca"))
