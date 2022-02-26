function checkInclusion(s1: string, s2: string): boolean {
  const mapInit = new Map<string, number>()

  for (let alpha of s1) {
    mapInit.set(alpha, mapInit.has(alpha) ? mapInit.get(alpha) + 1 : 1)
  }
  for (let i = 0; i <= s2.length - s1.length; i++) {
    let same = true
    const mapCompare = new Map<string, number>()
    let wordStr = s2.substr(i, s1.length)
    for (let j = 0; j < s1.length; j++) {
      if (mapInit.has(wordStr[j])) {
        mapCompare.set(
          wordStr[j],
          mapCompare.has(wordStr[j]) ? mapCompare.get(wordStr[j]) + 1 : 1
        )
      } else {
        break
      }
    }
    //如果两个表相等,那么就符合条件
    for (let [key, value] of mapInit) {
      if (mapCompare.get(key) !== value) {
        same = false
        break
      }
    }
    if (same) return true
  }
  return false
}

console.log(checkInclusion("ab", "eidboaoo"))
