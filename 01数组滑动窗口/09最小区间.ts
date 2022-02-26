function smallestRange(nums: number[][]) {
  let allNums: { num: number; type: number }[] = []
  let map: number[] = []
  let left = 0
  let count = 0
  let minLen = Infinity
  let minStart = 0
  //首先将数组按递增顺序扩展
  for (let i = 0; i < nums.length; i++) {
    map[i] = 0
    for (let j = 0; j < nums[i].length; j++) {
      allNums.push({ num: nums[i][j], type: i })
    }
  }
  allNums.sort((a, b) => a.num - b.num)

  for (let right = 0; right < allNums.length; right++) {
    if (map[allNums[right].type] === 0) count++
    //记录每个组有多少个数
    map[allNums[right].type]++
    //找到所有的目标数字
    while (count === nums.length && left <= right) {
      //找出比minLen更小的解
      if (allNums[right].num - allNums[left].num < minLen) {
        //更新
        minLen = allNums[right].num - allNums[left].num
        minStart = allNums[left].num
      }
      map[allNums[left].type]--
      if (map[allNums[left].type] === 0) count--
      left++
    }
  }
  return [minStart, minStart + minLen]
}
console.log(
  smallestRange([
    [13, 27, 1, 4],
    [9, 35, 16],
  ])
)
