/** @format */

//获取两个联合类型的差值
type Union<T, U> = Exclude<T, U> | Exclude<U, T>

type Diff<T extends {}, U extends {}> = {
  [P in Union<keyof T, keyof U>]: P extends keyof T
    ? T[P]
    : P extends keyof U
    ? U[P]
    : never
}

type D = Diff<{ a: string; b: number }, { b: string; c: string }>

export {}
