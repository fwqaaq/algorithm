/** @format */

type Last<T extends any[]> = T extends [...infer U, infer R]
  ? R extends any[]
    ? Last<R>
    : R
  : undefined

type arr1 = ["a", "b", "c"]
type arr2 = [3, 2, 1]

type tail1 = Last<arr1> // expected to be 'c'
type tail2 = Last<arr2> // expected to be 1

type S = Last<[arr1, [1, 2, [3]]]>
export {}
