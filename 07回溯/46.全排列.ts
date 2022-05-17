/**
 * /*
 *
 * [46] 全排列
 *
 * @format
 * @lc app=leetcode.cn id=46 lang=typescript
 */

// @lc code=start
function permute(nums: number[]): number[][] {
  const res: number[][] = []
  function trackback(repeatArr: boolean[], arr: number[]) {
    if (arr.length === nums.length) {
      res.push([...arr])
      return
    }

    for (let i = 0; i < nums.length; i++) {
      if (repeatArr[i] === true) continue
      //* 当开始排列时,将重复的数字过滤
      repeatArr[i] = true
      arr.push(nums[i])
      trackback(repeatArr, arr)
      arr.pop()
      //* 回溯的时候,将标记的位置改为false
      repeatArr[i] = false
    }
  }
  trackback([], [])
  return res
}
// @lc code=end
