/**
 * /*
 *
 * [47] 全排列 II
 *
 * @format
 * @lc app=leetcode.cn id=47 lang=typescript
 */

// @lc code=start
function permuteUnique(nums: number[]): number[][] {
  const res: number[][] = []
  nums.sort((a, b) => a - b)
  function trackback(repeatArr: boolean[], arr: number[]) {
    if (arr.length === nums.length) {
      res.push([...arr])
      return
    }

    for (let i = 0; i < nums.length; i++) {
      //* 当广度遍历开始时.并且上一个数字和当前数字相同时
      //并且上一个数字是未使用的状态,必须是false.为了避免递归的时候出现跳出当前循环的现象
      //例如到第三次递归的时候.i=0-->i=2,这时候没有false限制,会和使用状态的数字比较,跳出本次循环
      if (i > 0 && repeatArr[i - 1] === false && nums[i] === nums[i - 1])
        continue
      if (repeatArr[i] === true) continue
      repeatArr[i] = true
      arr.push(nums[i])
      trackback(repeatArr, arr)
      arr.pop()
      repeatArr[i] = false
    }
  }
  trackback([], [])
  return res
}
// @lc code=end

console.log(permuteUnique([3, 3, 0, 3]))

export {}
