/** @format */

type Trunc<T extends number> = `${T}` extends `${infer U}.${string}`
  ? U
  : `${T}`

type A = Trunc<12.34> // 12
export {}
