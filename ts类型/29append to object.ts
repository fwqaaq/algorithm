/** @format */

type AppendToObject<T, K extends string, V> = {
  //* key只有是T的子类型的时候才会返回T[key]
  [key in keyof T | K]: key extends keyof T ? T[key] : V
}

type Test = { id: "1" }
type Result = AppendToObject<Test, "value", 4> // expected to be { id: '1', value: 4 }

export {}
