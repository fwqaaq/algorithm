/** @format */

type Shift<T extends any[]> = T extends [infer U, ...infer A] ? A : never

type Result = Shift<[3, 2, 1]> // [2, 1]

export {}
