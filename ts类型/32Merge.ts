/** @format */

type Merge<O1 extends Object, O2 extends Object> = {
  [K in keyof O2 | keyof O1]: K extends keyof O2
    ? O2[K]
    : K extends keyof O1
    ? O1[K]
    : never
}

type foo = {
  name: string
  age: string
}
type coo = {
  age: number
  sex: string
}

type Result = Merge<foo, coo> // expected to be {name: string, age: number, sex: string}
export {}
