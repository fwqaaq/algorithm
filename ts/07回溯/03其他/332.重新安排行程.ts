/**
 * /*
 *
 * [332] 重新安排行程
 *
 * @format
 * @lc app=leetcode.cn id=332 lang=typescript
 */

// @lc code=start
function findItinerary(tickets: string[][]): string[] {
  const result = ["JFK"]
  const map: { [key: string]: Map<string, number> } = {}
  //重新按照从小到大的顺序排列
  tickets.sort((a, b) => {
    return a[1] < b[1] ? -1 : 1
  })
  for (const [from, to] of tickets) {
    if (!map[from]) map[from] = new Map()
    //将目的地设置为键,一个出发地可能对应多个不同的目的地
    map[from].set(to, (map[from].get(to) || 0) + 1)
  }

  function trackback(): boolean {
    //如果result的长度达到tickets长度的+1,就立刻结束
    if (result.length === tickets.length + 1) return true
    //* 遍历Map的目的地以及数量
    let pma = map[result[result.length - 1]]
    if (pma) {
      for (const [to, count] of pma.entries()) {
        if (count > 0) {
          result.push(to)
          pma.set(to, count - 1)
          if (trackback()) return true
          pma.set(to, count)
          result.pop()
        }
      }
    }
    return false
  }
  trackback()
  return result
}
// @lc code=end
console.log(
  findItinerary([
    ["MUC", "LHR"],
    ["JFK", "MUC"],
    ["SFO", "SJC"],
    ["LHR", "SFO"],
  ])
)
export {}
