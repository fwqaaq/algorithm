/** @format */

const tree1 = {
  val: 1,
  left: null,
  right: {
    val: 2,
    left: {
      val: 3,
      left: null,
      right: null,
    },
    right: null,
  },
} as const

interface TreeNode {
  val: number
  left: TreeNode | null
  right: TreeNode | null
}

type InorderTraversal<
  T extends TreeNode | null,
  S extends NonNullable<T> = NonNullable<T>
> = [T] extends [S]
  ? [...InorderTraversal<S["left"]>, S["val"], ...InorderTraversal<S["right"]>]
  : []

type A = InorderTraversal<typeof tree1> // [1, 3, 2]

export {}
