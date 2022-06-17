/** @format */

type Fill<
  T extends any[],
  V extends number,
  A extends any[] = []
> = A["length"] extends T["length"] ? A : Fill<T, V, [...A, V]>

type exp = Fill<[1, 2, 3], 0> // expected to be [0, 0, 0]

export {}
