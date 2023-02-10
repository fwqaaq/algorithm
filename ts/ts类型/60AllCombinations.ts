/** @format */

type UnionString<S extends string> = S extends `${infer U}${infer T}`
  ? U | UnionString<T>
  : never

type AllCombinations<S extends string, U extends string = UnionString<S>> = [
  U
] extends [never]
  ? ""
  : "" | { [K in U]: `${K}${AllCombinations<never, Exclude<U, K>>}` }[U]

type AllCombinations_ABC = AllCombinations<"ABC">
// should be '' | 'A' | 'B' | 'C' | 'AB' | 'AC' | 'BA' | 'BC' | 'CA' | 'CB' | 'ABC' | 'ACB' | 'BAC' | 'BCA' | 'CAB' | 'CBA'
export {}
