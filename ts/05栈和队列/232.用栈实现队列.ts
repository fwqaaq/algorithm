/*
 * @lc app=leetcode.cn id=232 lang=typescript
 *
 * [232] 用栈实现队列
 */

// @lc code=start
/* A queue implemented using two stacks */
/* A queue implemented using two stacks */
class MyQueueM<T> {
  stackin: T[]
  stackout: T[]
  constructor() {
    this.stackin = []
    this.stackout = []
  }

  push(x: T): void {
    this.stackin.push(x)
  }

  pop(): T {
    if (this.stackout.length === 0) {
      while (this.stackin.length > 0) {
        this.stackout.push(this.stackin.pop()!)
      }
    }
    return this.stackout.pop()!
  }

  peek(): T {
    let temp = this.pop()
    this.stackout.push(temp)
    return temp
  }

  empty(): boolean {
    return this.stackin.length === 0 && this.stackout.length === 0
  }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * var obj = new MyQueue()
 * obj.push(x)
 * var param_2 = obj.pop()
 * var param_3 = obj.peek()
 * var param_4 = obj.empty()
 */
// @lc code=end
