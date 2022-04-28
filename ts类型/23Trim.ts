/** @format */

type TrimLeft<T extends string> = T extends ` ${infer U}` ? TrimLeft<U> : T
type TrimRight<T extends string> = T extends `${infer U} ` ? TrimRight<U> : T
type Trim<T extends string> = TrimLeft<TrimRight<T>>

type trimmed = Trim<"  Hello World       "> // expected to be 'Hello World'

export {}
