/** @format */

interface Todo {
  title: string
  description: string
  completed: boolean
}

type Exclude<T, U> = T extends U ? never : T
type MyReadonly2<K extends {}, V extends keyof K> = {
  readonly //* 不需要使用keyof,因为V已经是一个联合类型
  [P in V]: K[P]
} & {
  [P in Exclude<keyof K, V>]: K[P]
}

const todo: MyReadonly2<Todo, "title" | "description"> = {
  title: "Hey",
  description: "foobar",
  completed: false,
}

todo.title = "Hello" // Error: cannot reassign a readonly property
todo.description = "barFoo" // Error: cannot reassign a readonly property
todo.completed = true // OK
export {}
