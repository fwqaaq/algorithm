/**
 * /*
 *
 * [559] N 叉树的最大深度
 *
 * @format
 * @lc app=leetcode.cn id=559 lang=typescript
 */

// @lc code=start

class Node {
  val: number
  children: Node[]
  constructor(val?: number, children?: Node[]) {
    this.val = val === undefined ? 0 : val
    this.children = children === undefined ? [] : children
  }
}

function maxDepth(root: Node | null): number {
  // if (root === null) return 0
  // let max = 0
  // //* 由于最后一个叶节点,没有children属性,所以不会被遍历到,也不可能++
  // for (let i = 0; i < root.children.length; i++) {
  // //* 尾调用优化不可取,所以在这里+1,会导致最后一个叶节点没有遍历到
  // max = Math.max(max, maxDepth(root.children[i]))
  // }
  // return max + 1
  if (root === null) return 0
  let queue: Node[] = [root]
  let count = 0
  while (queue.length !== 0) {
    count++
    let len = queue.length
    for (let i = 0; i < len; i++) {
      let curNode = queue.shift()
      //* 不能在if中进行shfit,这样最后一个叶子节点由于没有children属性,不可能被遍历到,也就不会弹出
      //* 导致死循环
      if (curNode!.children.length !== 0) {
        curNode?.children.forEach((item) => {
          queue.push(item)
        })
      }
    }
  }
  return count
}
// @lc code=end

const root = new Node(1, [
  new Node(3, [new Node(5, [new Node(6, [new Node(7)])])]),
  new Node(2),
  new Node(4),
])

console.log(maxDepth(root))
export {}
