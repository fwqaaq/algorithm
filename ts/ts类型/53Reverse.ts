/** @format */

type Reverse<T extends any[], V extends any[] = []> = T extends [
  infer F,
  ...infer R
]
  ? Reverse<R, [F, ...V]>
  : V

type a = Reverse<["a", "b"]> // ['b', 'a']
type b = Reverse<["a", "b", "c"]> // ['c', 'b', 'a']

export { Reverse }
