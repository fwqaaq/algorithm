/** @format */

type TrimRight<T extends string> = T extends `${infer U} ` ? TrimRight<U> : T

type Trimed = TrimRight<"   Hello World    "> // expected to be '   Hello World'

export {}
