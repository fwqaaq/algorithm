/*
 * @lc app=leetcode.cn id=225 lang=rust
 *
 * [225] 用队列实现栈
 */

// @lc code=start
struct MyStack {
    queue: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        MyStack { queue: vec![] }
    }

    fn push(&mut self, x: i32) {
        self.queue.push(x);
    }

    fn pop(&mut self) -> i32 {
        let len = self.queue.len() - 1;
        for _ in 0..len {
            let tmp = self.queue.remove(0);
            self.queue.push(tmp);
        }
        self.queue.remove(0)
    }

    fn top(&mut self) -> i32 {
        let res = self.pop();
        self.queue.push(res);
        res
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}
// @lc code=end
