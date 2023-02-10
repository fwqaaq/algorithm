/** @format */

type Capitalize<T extends string> = T extends `${infer U}${infer V}`
  ? `${Uppercase<U>}${V}`
  : T

type capitalized = Capitalize<"hello world"> // expected to be 'Hello world'

export {}
