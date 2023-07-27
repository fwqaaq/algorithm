/*
 * @lc app=leetcode.cn id=209 lang=c
 *
 * [209] 长度最小的子数组
 */

// @lc code=start
int minSubArrayLen(int target, int *nums, int numsSize) {
    int result = __INT_MAX__;
    int left = 0;
    int sum = 0;
    for (int right = 0; right < numsSize; right++) {
        sum += nums[right];
        while (sum >= target) {
            int len = right - left + 1;
            result = len <= result ? len : result;
            sum -= nums[left++];
        }
    }
    return result == __INT_MAX__ ? 0 : result;
}
// @lc code=end
