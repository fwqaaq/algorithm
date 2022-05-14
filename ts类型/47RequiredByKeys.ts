/** @format */

interface User {
  name?: string
  age?: number
  address?: string
}

type RequiredByKeys<T, K extends keyof T> = {
  [P in K]-?: T[P]
} & {
  [P in Exclude<keyof T, K>]?: T[P]
}

type UserPartialName = RequiredByKeys<User, "name"> // { name: string; age?: number; address?: string }

export {}
