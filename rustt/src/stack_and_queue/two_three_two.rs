/*
 * @lc app=leetcode.cn id=232 lang=rust
 *
 * [232] 用栈实现队列
 */

// @lc code=start
struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack_in: Vec::new(),
            stack_out: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            while !self.stack_in.is_empty() {
                self.stack_out.push(self.stack_in.pop().unwrap());
            }
        }
        self.stack_out.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        let res = self.pop();
        self.stack_out.push(res);
        res
    }

    fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_my_queue() {
        let mut obj = MyQueue::new();
        obj.push(2);
        obj.push(3);
        obj.push(4);
        assert_eq!(obj.pop(), 2);
        assert_eq!(obj.peek(), 3);
        assert!(!obj.empty());
    }
}
