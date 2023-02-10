/** @format */

interface Model {
  name: string
  age: number
  locations: string[] | null
}

type ObjectEntries<T extends {}, U = keyof T> = U extends U
  ? [U] extends [keyof T]
    ? [U, T[U]]
    : never
  : never

type modelEntries = ObjectEntries<Model> // ['name', string] | ['age', number] | ['locations', string[] | null];

export {}
