/** @format */

type Permutation<T, K = T> = [T] extends [never]
  ? []
  : T extends T
  ? //* 条件类型,一直递归到最后一个元素(T = never)返回空数组
    [T, ...Permutation<Exclude<K, T>>]
  : never

type perm = Permutation<"A" | "B" | "C"> // ['A', 'B', 'C'] | ['A', 'C', 'B'] | ['B', 'A', 'C'] | ['B', 'C', 'A'] | ['C', 'A', 'B'] | ['C', 'B', 'A']

export {}
