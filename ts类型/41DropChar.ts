/** @format */

type DropChar<
  T extends string,
  D extends string
> = T extends `${infer F}${infer R}`
  ? //如果F===D,那么去除相同的字符,否则保留
    F extends D
    ? DropChar<R, D>
    : `${F}${DropChar<R, D>}`
  : //* 递归到终点的时候返回""
    ""

type Butterfly = DropChar<" b u t t e r f l y ! ", " "> // 'butterfly!'

export {}
