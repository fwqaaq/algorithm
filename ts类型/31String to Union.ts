/** @format */

type StringToUnion<T extends string> = T extends `${infer F}${infer L}`
  ? F | StringToUnion<L>
  : never
type Test = "123"
type Result = StringToUnion<Test> // expected to be "1" | "2" | "3"

export {}
