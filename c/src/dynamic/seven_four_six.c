/**
 *
 *
 * [746] 使用最小花费爬楼梯
 *
 * @format
 * @lc app=leetcode.cn id=746 lang=c
 */

// @lc code=start
#include <math.h>
int minCostClimbingStairs(int *cost, int costSize) {
    // int dp[costSize + 1];
    // dp[0] = dp[1] = 0;
    // for (int i = 2; i <= costSize; i++) {
    //   dp[i] = fmin(dp[i - 2] + cost[i - 2], dp[i - 1] + cost[i - 1]);
    // }
    // return dp[costSize];

    int dpBefore = 0, dpAfter = 0;
    for (int i = 2; i <= costSize; i++) {
        int dpi = fmin(dpBefore + cost[i - 2], dpAfter + cost[i - 1]);
        dpBefore = dpAfter;
        dpAfter = dpi;
    }
    return dpAfter;
}

// @lc code=end
