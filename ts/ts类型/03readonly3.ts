/** @format */

type X = {
  x: {
    a: 1
    b: "hi"
  }
  y: "hey"
}

type DeepReadonly<K extends {}> = K extends {}
  ? { readonly [P in keyof K]: DeepReadonly<K[P]> }
  : K

type Expected = {
  readonly x: {
    readonly a: 1
    readonly b: "hi"
  }
  readonly y: "hey"
}

type Todo = DeepReadonly<X> // should be same as `Expected`

export {}
