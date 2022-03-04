/*
 * @lc app=leetcode.cn id=350 lang=typescript
 *
 * [350] 两个数组的交集 II
 */

// @lc code=start
function intersect(nums1: number[], nums2: number[]): number[] {
  let shortArr = nums1.length > nums2.length ? nums2 : nums1
  let longtArr = nums1.length > nums2.length ? nums1 : nums2
  let numArr: number[] = []
  for (let i = 0; i < shortArr.length; i++) {
    //获得长数组中的交集元素的索引,并且将他改为1
    let index = longtArr.indexOf(shortArr[i])
    if (index !== -1) {
      numArr.push(shortArr[i])
      //短数组,长数组的元素都需要改为-1,以防再次出现重复元素
      shortArr[i] = -1
      longtArr[index] = -1
    }
  }
  return numArr
}
// @lc code=end

console.log(intersect([4, 9, 5], [9, 4, 9, 8, 4]))
