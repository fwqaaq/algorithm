/** @format */

type ReplaceAll<
  T extends string,
  K extends string,
  V extends string
> = T extends `${infer L}${K}${infer R}` ? ReplaceAll<`${L}${V}${R}`, K, V> : T
type replaced = ReplaceAll<"t y p e s", " ", ""> // expected to be 'types'

export {}
