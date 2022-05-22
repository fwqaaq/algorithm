/** @format */

import { Reverse } from "./53Reverse"

type FlipArguments<T extends Function> = T extends (...args: infer A) => infer S
  ? (...args: Reverse<A>) => S
  : never

type Flipped = FlipArguments<
  (arg0: string, arg1: number, arg2: boolean) => void
>
// (arg0: boolean, arg1: number, arg2: string) => void
export {}
