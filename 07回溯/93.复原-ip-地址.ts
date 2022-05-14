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
    //剪枝优化
    if (strArr.length === 4) return

    let temp = ""
    //剪枝优化: i和startIndex的范围不能超过2
    for (let i = startIndex; i < s.length, i - startIndex < 3; i++) {
      //* temp的最大长度就是3,所以i+1-startIndex<=3
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
console.log(restoreIpAddresses("25525511135"))
