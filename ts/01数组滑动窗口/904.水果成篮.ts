/*
 * @lc app=leetcode.cn id=904 lang=typescript
 *
 * [904] 水果成篮
 */

// @lc code=start
function totalFruit(fruits: number[]): number {
  let right = 0,
    left = 0,
    max = 0
  while (right < fruits.length) {
    // 保证右边的窗口中只有两种水果,每超过一次,重新更换窗口
    let set = new Set<number>()
    while (set.size <= 2) {
      set.add(fruits[right])
      //确保右边的窗口不会超过数组的长度且有三个水果不会继续扩张
      if (set.size === 3 || right === fruits.length) break
      right++
    }
    max = Math.max(max, right - left)
    //如果长度超过数组长度,直接退出循环
    if (right === fruits.length) break
    left = right
    //使用左侧窗口更新右窗口,确定没有遗漏的情况
    while (fruits[left - 1] === fruits[right - 1]) {
      //只要左右两侧的水果相同,就继续更新右侧窗口,直到不同
      right--
    }
    //使左侧窗口等于右侧窗口
    left = right
  }
  return max
}
// @lc code=end
console.log(totalFruit([3, 3, 3, 1, 2, 1, 1, 1, 3, 3, 3, 4]))
