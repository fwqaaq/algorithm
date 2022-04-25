/** @format */

type Parameters<T extends (...args: any) => any> = T extends (
  ...args: infer P
) => any
  ? P
  : never

type x = Parameters<(a: string, b: number) => string>

export {}
