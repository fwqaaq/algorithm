/*
 * @lc app=leetcode.cn id=225 lang=typescript
 *
 * [225] 用队列实现栈
 */

// @lc code=start
class MyStack<T> {
  queue: T[]
  constructor() {
    this.queue = []
  }

  push(x: T): void {
    this.queue.push(x)
  }

  pop(): T | null {
    if (this.queue.length === 0) {
      return null
    }
    return this.queue.pop()!
  }

  top(): T | null {
    if (this.queue.length !== 0) {
      return this.queue[this.queue.length - 1]
    }
    return null
  }

  empty(): boolean {
    return this.queue.length === 0
  }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * var obj = new MyStack()
 * obj.push(x)
 * var param_2 = obj.pop()
 * var param_3 = obj.top()
 * var param_4 = obj.empty()
 */
// @lc code=end
