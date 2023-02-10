/**
 * /*
 *
 * [39] 组合总和
 *
 * @format
 * @lc app=leetcode.cn id=39 lang=typescript
 */

// @lc code=start
function combinationSum(candidates: number[], target: number): number[][] {
  const result: number[][] = []

  function recursive(startIndex: number, arr: number[], sum: number) {
    //终止条件,符合target
    if (sum === target) {
      result.push([...arr])
      return
    }
    //* 终止条件,超过范围
    if (sum > target) return

    //sum + candidates[i] <= target: 剪枝优化,只要大于target就不用继续遍历
    for (
      let i = startIndex;
      i < candidates.length && sum + candidates[i] <= target;
      i++
    ) {
      arr.push(candidates[i])
      recursive(i, arr, sum + candidates[i])
      arr.pop()
    }
  }
  recursive(0, [], 0)
  return result
}
// @lc code=end

console.log(combinationSum([2, 3, 6, 7], 7))
