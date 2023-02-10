/**
 * /*
 *
 * [118] 杨辉三角
 *
 * @format
 * @lc app=leetcode.cn id=118 lang=typescript
 */

// @lc code=start
function generate(numRows: number): number[][] {
  const res: number[][] = []
  function recursive(num: number, arr: number[]) {
    res.push([...arr])
    if (numRows === num + 1) return
    let temp: number = 1
    for (let i = 0; i < num; i++) {
      let tem = arr[i + 1]
      arr[i + 1] += temp
      temp = tem
    }
    arr.push(1)
    recursive(num + 1, arr)
  }
  recursive(0, [1])
  return res
}
// @lc code=end
console.log(generate(5))
