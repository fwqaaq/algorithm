/** @format */

type Fibonacci<
  T extends number,
  Index extends number[] = [1],
  P extends number[] = [],
  C extends number[] = [1]
> = Index['length'] extends T
  ? C['length']
  : Fibonacci<T, [...Index, 1], C, [...C, ...P]>

type Result1 = Fibonacci<3> // 2
type Result2 = Fibonacci<8> // 21

export {}
