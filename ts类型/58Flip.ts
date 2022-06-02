/** @format */

type Flip<T extends Record<string, string | boolean | number>> = {
  [K in keyof T as `${T[K]}`]: K
}

type x = Flip<{ a: "x"; b: "y"; c: "z" }> // {x: 'a', y: 'b', z: 'c'}
type x1 = Flip<{ a: 1; b: 2; c: 3 }> // {1: 'a', 2: 'b', 3: 'c'}
type x2 = Flip<{ a: false; b: true }> // {false: 'a', true: 'b'}

export {}
