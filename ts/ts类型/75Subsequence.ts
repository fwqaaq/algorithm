/** @format */

type Subsequence<T extends any[]> = T extends [infer F, ...infer L]
  ? [F, ...Subsequence<L>] | Subsequence<L>
  : T

type A = Subsequence<[1, 2]> // [] | [1] | [2] | [1, 2]

export {}
