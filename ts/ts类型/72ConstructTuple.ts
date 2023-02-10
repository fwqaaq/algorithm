/** @format */

type ConstructTuple<
  T extends number,
  R extends any[] = []
> = R['length'] extends T ? R : ConstructTuple<T, [...R, unknown]>

type result = ConstructTuple<2> // expect to be [unknown, unkonwn]

export {}
