/** @format */

type Without<T extends number[], V extends number[] | number> = V

type Res = Without<[1, 2], 1> // expected to be [2]
type Res1 = Without<[1, 2, 4, 1, 5], [1, 2]> // expected to be [4, 5]
type Res2 = Without<[2, 3, 2, 3, 2, 3, 2, 3], [2, 3]> // expected to be []

type res = [[1, 2, 3]] extends [[1, 2, 3], [1, 2, 3]] ? true : false

export {}
