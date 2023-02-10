/** @format */

interface Todo {
  readonly title: string
  readonly description: string
  readonly completed: boolean
}

type Mutable<T extends {}> = {
  -readonly [P in keyof T]: T[P]
}

type MutableTodo = Mutable<Todo> // { title: string; description: string; completed: boolean; }

export {}
