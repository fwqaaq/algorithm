/** @format */

type RemoveIndexSignature<T> = {
  [K in keyof T as K extends String ? K : never]: T[K]
}

type Foo = {
  [key: number]: string
  //* 函数签名是string类型
  foo(): void
}

type A = RemoveIndexSignature<Foo> // expected { foo(): void }
export {}
