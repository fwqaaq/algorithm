/** @format */

//* 递归的去重复TrimLeft,只要T左没有" ",就不可能推到出U
type TrimLeft<T extends string> = T extends ` ${infer U}` ? TrimLeft<U> : T

type trimed = TrimLeft<"    Hello World  "> // expected to be 'Hello World  '

export {}
