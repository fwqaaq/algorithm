/** @format */

type Chainable<T = {}> = {
  option<K extends string, V>(
    //* 如果当前的T排除之前链式调用的K,返回当前的K
    key: K extends keyof T ? never : K,
    value: V
    //* 递归的T与当前的K的交叉类型
  ): Chainable<T & { [key in K]: V }>
  get(): T
}
declare const config: Chainable

const result = config
  .option("foo", 123)
  .option("name", "type-challenges")
  .option("bar", { value: "Hello World" })
  .get()

// expect the type of result to be:
interface Result {
  foo: number
  name: string
  bar: {
    value: string
  }
}

export {}
