/**
 * /*
 *
 * [93] 复原 IP 地址
 *
 * @format
 * @lc app=leetcode.cn id=93 lang=typescript
 */

// @lc code=start
function isValidIp(s: string) {
  let resbool = true
  let temp = Number(s)
  if (
    s.length === 0 ||
    isNaN(temp) ||
    temp < 0 ||
    temp > 255 ||
    (s.length > 1 && s[0] === "0")
  ) {
    resbool = false
  }
  return resbool
}
function restoreIpAddresses(s: string): string[] {
  let res: string[] = []

  function trackback(startIndex: number, strArr: string[]) {
    if (4 === strArr.length && startIndex === s.length) {
      res.push(strArr.join("."))
      return
    }

    let temp = ""
    for (let i = startIndex; i < Math.min(s.length, startIndex + 3); i++) {
      temp = s.slice(startIndex, i + 1)
      if (isValidIp(temp)) {
        strArr.push(temp)
        trackback(i + 1, strArr)
        strArr.pop()
      }
    }
  }
  trackback(0, [])
  return res
}
// @lc code=end
