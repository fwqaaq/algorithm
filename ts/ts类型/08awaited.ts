/** @format */

type x = Awaited<Promise<Promise<string>>>

type Awaited<T> = T extends undefined | null
  ? T
  : T extends object & { then: (resolve: infer V) => any }
  ? V extends (value: infer U) => any
    ? Awaited<U>
    : never
  : T

export {}
