/** @format */

interface User {
  name: string
  age: number
  address: string
}

type PartialByKeys<T, K extends keyof T> = {
  [P in K]?: T[P]
} & {
  [V in Exclude<keyof T, K>]: T[V]
}

type UserPartialName = PartialByKeys<User, "name"> // { name?:string; age:number; address:string }

const x: UserPartialName = {
  age: 12,
  address: "s",
}

export {}
