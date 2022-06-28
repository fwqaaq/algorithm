/** @format */

type Combination<
  T extends string[],
  K extends string = T[number],
  S extends string = K
> = [K] extends [never]
  ? ''
  : K extends K
  ? K | `${K} ${Combination<T, Exclude<S, K>>}`
  : never

type Keys = Combination<['foo', 'bar', 'baz']>

export {}
