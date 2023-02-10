/**
 * /*
 *
 * [134] 加油站
 *
 * @format
 * @lc app=leetcode.cn id=134 lang=typescript
 */

// @lc code=start
function canCompleteCircuit(gas: number[], cost: number[]): number {
  let sum = 0
  let total = 0
  let start = 0
  for (let i = 0; i < gas.length; i++) {
    sum += gas[i] - cost[i]
    //同时记录所有的总和
    total += gas[i] - cost[i]
    if (sum < 0) {
      start = i + 1
      sum = 0
    }
  }
  //总数不等于0,说明cost>gas,不能完成
  if (total < 0) return -1
  return start
}
// @lc code=end
