/** @format */
type IsTuple<T extends readonly any[]> = number extends T["length"]
  ? false
  : true
// readonly 表示的类型更宽松,不具体,所以只能接收readonly类型
type case1 = IsTuple<[number]> // true
type case2 = IsTuple<readonly [number]> // true
type case3 = IsTuple<readonly number[]> // false

export {}
