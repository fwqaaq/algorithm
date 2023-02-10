/*
 * 有n件物品和一个最多能背重量为w 的背包。第i件物品的重量是weight[i]，得到的价值是value[i]
 * 每件物品只能用一次，求解将哪些物品装入背包里物品价值总和最大。
 * dp[i][j]: 前i个物品，背包容量为j，能获得的最大价值
 * dp[0][*]: u=weight[0],u之前为0，u之后（含u）为value[0]
 * dp[*][0]: 0
 * ...
 * dp[i][j]: max(dp[i-1][j], dp[i-1][j-weight[i]]+value[i]);
 *
 * @format
 */

// function testWeightbagTest(weight: number[], value: number[], size: number) {
//   const goods = weight.length
//   const dp: number[][] = new Array(goods)
//     .fill(0)
//     .map(() => new Array(size + 1).fill(0))
//   //todo: initialize array, startup weight[0] === dp[0][i]
//   for (let i = weight[0]; i < size + 1; i++) {
//     dp[0][i] = value[0]
//   }
//   /*
//    * @params: i --> the goods' number
//    * @params: j --> the size of the bag
//    */
//   for (let i = 1; i < goods; i++) {
//     for (let j = 1; j < size + 1; j++) {
//       if (j < weight[i]) dp[i][j] = dp[i - 1][j]
//       // beacuse the vlaue of current weight[i] correspond with value[i]
//       // so we need to find the most suitable value, j - weight[i]
//       else
//         dp[i][j] = Math.max(dp[i - 1][j], dp[i - 1][j - weight[i]] + value[i])
//     }
//   }
//   return dp[goods - 1][size]
// }

//*滚动数组-----------------------------------------------------
/*
 * 使用滚动数组，第一层for循环控制物品
 * i = 0 ---> 将会得到 0 1 1 1 1
 * i = 1 ---> 将会在原属组的基础上比较之后得到 0 1 1 5 6
 * ..., 每次循环都会使用上一次的数组进行更新
 * dp[j - weight[i]] 在通过的前提下，获得上一次可以放在 j - weight[i] 位置的值（此值一定是最大的）
 *    当然这要和上次此位置的值比较 dp[j] 
 TODO: 为什么不正序遍历
 dp[1] = dp[1 - weight[0]] + value[0] =value[0]
 dp[2] = dp[2 - weight[0]] + value[0]
       = dp[1] + value[0]
 * 此时 value[0],即物品0会算两次，会出问题
 */
function testWeightbagTest(weight: number[], value: number[], size: number) {
  const dp: number[] = new Array(size + 1).fill(0)
  for (let i = 0; i < weight.length; i++) {
    for (let j = size; j >= weight[i]; j--) {
      dp[j] = Math.max(dp[j], dp[j - weight[i]] + value[i])
    }
  }
  return dp[size]
}

const weight = [1, 3, 4]
const value = [1, 5, 7]
const size = 4
console.log(testWeightbagTest(weight, value, size))

export {}
