/** @format */

type GreaterThan<
  T extends number,
  V extends number,
  A extends any[] = [],
  B extends any[] = []
> = A["length"] extends T
  ? false
  : A["length"] extends V
  ? true
  : GreaterThan<T, V, [...A, V], [...B, V]>

type result1 = GreaterThan<2, 1> //should be true
type result2 = GreaterThan<1, 1> //should be false
type result3 = GreaterThan<10, 100> //should be false
type result4 = GreaterThan<111, 11> //should be true

export {}
