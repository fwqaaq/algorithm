/** @format */

type Absolute<T extends number> = `${T}` extends `${infer F}${infer L}`
  ? L
  : never

type Test = -100
type Result = Absolute<Test> // expected to be "100"

export {}
