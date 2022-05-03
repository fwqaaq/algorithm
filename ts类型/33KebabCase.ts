/** @format */

//* 建立一个小写类型+连字符
type MyLowercase<T extends string> = Lowercase<T> extends T
  ? Capitalize<T> extends T
    ? T
    : Lowercase<T>
  : `-${Lowercase<T>}`

type KebabCase<S extends string, head = true> = S extends `${infer A}${infer B}`
  ? head extends true
    ? // 开头字母小写,剩下的字母通过MyLowercase转换
      `${Lowercase<A>}${KebabCase<B, false>}`
    : `${MyLowercase<A>}${KebabCase<B, false>}`
  : S

export {}
