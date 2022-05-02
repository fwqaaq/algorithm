/** @format */

type Len<S extends string, R extends any[]> = S extends `${infer F}${infer L}`
  ? Len<L, [...R, F]>
  : R["length"]

type x = Len<"ABC", []> // 3

export {}
