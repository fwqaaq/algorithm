/** @format */

type PickByType<T, U> = {
  [P in keyof T as T[P] extends U ? P : never]: T[P]
}

type OnlyBoolean = PickByType<
  {
    name: string
    count: number
    isReadonly: boolean
    isEnable: boolean
  },
  boolean
> // { isReadonly: boolean; isEnable: boolean; }
export {}
