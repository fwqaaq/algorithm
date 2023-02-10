/** @format */

type Zip<T extends any[], U extends any[]> = T extends [infer A, ...infer RA]
  ? U extends [infer B, ...infer RB]
    ? [[A, B], ...Zip<RA, RB>]
    : []
  : []

type exp = Zip<[1, 2, 3, 3], [true, false, "T", "S"]> // expected to be [[1, true], [2, false]]

export {}
