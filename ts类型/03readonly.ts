/** @format */

interface Todo {
  readonly title: string
  readonly description: string
}

type MyReadonly<T extends {}> = {
  -readonly [P in keyof T]: T[P]
}

const todo: MyReadonly<Todo> = {
  title: "Hey",
  description: "foobar",
}

todo.title = "Hello" // Error: cannot reassign a readonly property
todo.description = "barFoo" // Error: cannot reassign a readonly property

export {}
