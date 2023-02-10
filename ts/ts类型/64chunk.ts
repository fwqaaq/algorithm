/** @format */

type Chunk<
  T extends unknown[],
  Len extends number,
  Item extends any[] = [],
  Res extends any[] = []
> = T extends [infer L, ...infer Rest]
  ? Item["length"] extends Len
    ? Chunk<T, Len, [], [...Res, Item]>
    : Chunk<Rest, Len, [...Item, L], Res>
  : [...Res, ...(Item extends [] ? [] : [Item])]

type exp1 = Chunk<[1, 2, 3], 2> // expected to be [[1, 2], [3]]
type exp2 = Chunk<[1, 2, 3], 4> // expected to be [[1, 2, 3]]
type exp3 = Chunk<[1, 2, 3], 1> // expected to be [[1], [2], [3]]
export {}
