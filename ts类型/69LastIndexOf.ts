/** @format */

type LastIndexOf<
  T extends any[],
  N extends number,
  Arr extends any[] = T
> = Arr['length'] extends N
  ? T[N]
  : LastIndexOf<T, N, Arr extends [...infer F, infer R] ? F : []>

type Res1 = LastIndexOf<[1, 2, 3, 2, 1], 2> // 3
type Res2 = LastIndexOf<[0, 0, 0], 2> // -1

export {}
