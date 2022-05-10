/** @format */

type MinusOne<T extends number, R extends number[] = []> = T extends R["length"]
  ? R extends [infer F, ...infer L]
    ? L["length"]
    : 0
  : MinusOne<T, [T, ...R]>

type Zero = MinusOne<1> // 0
type FiftyFour = MinusOne<55> // 54

export {}
