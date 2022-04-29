/** @format */

type AppendArgument<
  F extends (...args: any) => any,
  T extends any
> = F extends (...args: infer A) => any
  ? (...args: [...A, T]) => ReturnType<F>
  : F
type Fn = (a: number, b: string) => number

type Result = AppendArgument<Fn, boolean>
// expected be (a: number, b: string, x: boolean) => number
export {}
