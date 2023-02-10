/** @format */

type AnyOf<T extends any[]> = T extends [infer F, ...infer L]
  ? [F] extends [0 | "" | false | [] | { [key: string]: never }]
    ? AnyOf<L>
    : true
  : false
type Sample1 = AnyOf<[1, "", false, [], {}]> // expected to be true.
type Sample2 = AnyOf<[0, "", false, [], {}]> // expected to be false.

export {}
