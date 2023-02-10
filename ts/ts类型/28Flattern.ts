/** @format */

type Flatten<T extends any[]> = T extends [infer F, ...infer L]
  ? F extends any[]
    ? [...Flatten<F>, ...Flatten<L>]
    : [F, ...Flatten<L>]
  : T

type flatten = Flatten<[1, 2, [3, 4], [[[5]]]]> // [1, 2, 3, 4, 5]

export {}
