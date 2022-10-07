/*
* @lc app=leetcode.cn id=49 lang=rust
*
* [49] 字母异位词分组
*/
pub struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut record = HashMap::new();
        strs.into_iter().for_each(|x| {
            let mut key = x.clone().into_bytes();
            key.sort();
            record.entry(key).or_insert(vec![]).push(x);
        });
        record.values().cloned().collect()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_group_anagrams() {
        let res = Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);
        println!("{:?}", res);
    }
}
