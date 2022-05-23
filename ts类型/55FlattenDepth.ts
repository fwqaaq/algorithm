/** @format */

import { type } from "os"

type FlatternOnce<T extends any[]> = T extends [infer F, ...infer L]
  ? //如果F不是数组-->[F],如果F是数组,推导为K
    [...(F extends [...infer K] ? K : [F]), ...FlatternOnce<L>]
  : T

type x = FlatternOnce<[[1], 2, 3]>

type FlattenDepth<
  T extends any[],
  V extends number = 1,
  P extends any[] = []
> = T extends any[]
  ? P["length"] extends V
    ? T
    : FlattenDepth<FlatternOnce<T>, V, [...P, V]>
  : never

type a = FlattenDepth<[1, 2, [3, 4], [[[5]]]], 2> // [1, 2, 3, 4, [5]]. flattern 2 times
type b = FlattenDepth<[1, 2, [3, 4], [[[5]]]]> // [1, 2, 3, 4, [[5]]]. Depth defaults to be 1

export {}
