/** @format */
//* 条件分布式类型,会将T的类型一个一个的传入比较
//此时的S=string | number,而T = string或者 T = number
//只有条件式的第一个类型([S])会被分布式
type IsUnion<T, S = T> = T extends S ? ([S] extends [T] ? false : true) : false

type case1 = IsUnion<string> // false
type case2 = IsUnion<string | number> // true
type case3 = IsUnion<[string | number]> // false

type a = string extends number | string ? true : false

export {}
