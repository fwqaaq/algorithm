/*
* @lc app=leetcode.cn id=406 lang=rust
*
* [406] 根据身高重建队列
*/
pub struct Solution;
// @lc code=start
use std::collections::LinkedList;
impl Solution {
    // pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    //     let mut queue = vec![];
    //     people.sort_by(|a, b| {
    //         if a[0] == b[0] {
    //             return a[1].cmp(&b[1]);
    //         }
    //         b[0].cmp(&a[0])
    //     });
    //     queue.push(people[0].clone());
    //     for v in people.iter().skip(1) {
    //         queue.insert(v[1] as usize, v.clone());
    //     }
    //     queue
    // }

    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut queue = LinkedList::new();

        people.sort_by(|a, b| {
            if a[0] == b[0] {
                return a[1].cmp(&b[1]);
            }
            b[0].cmp(&a[0])
        });
        queue.push_back(people[0].clone());
        for v in people.iter().skip(1) {
            if queue.len() > v[1] as usize {
                let mut back_link = queue.split_off(v[1] as usize);
                queue.push_back(v.clone());
                queue.append(&mut back_link);
            } else {
                queue.push_back(v.clone());
            }
        }
        queue.into_iter().collect()
    }
}
// @lc code=end
