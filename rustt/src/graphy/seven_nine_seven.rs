/*
 * @lc app=leetcode.cn id=797 lang=rust
 *
 * [797] 所有可能的路径
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (mut res, mut path) = (vec![], vec![0]);
        Self::dfs(&graph, &mut path, &mut res, 0);
        res
    }

    pub fn dfs(graph: &Vec<Vec<i32>>, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, node: usize) {
        if node == graph.len() - 1 {
            res.push(path.clone());
            return;
        }
        for &v in &graph[node] {
            path.push(v);
            Self::dfs(graph, path, res, v as usize);
            path.pop();
        }
    }
}
// @lc code=end
