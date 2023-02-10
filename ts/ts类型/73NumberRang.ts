/** @format */

type NumberRange<
  T extends number,
  U extends number,
  R extends any[] = [],
  L extends any[] = [],
  S extends any[] = []
> = T extends R['length']
  ? U extends L['length']
    ? S[number] | L['length']
    : NumberRange<T, U, R, [...L, T], [...S, L['length']]>
  : NumberRange<T, U, [...R, T], [...R, T], S>
type result = NumberRange<1, 9> //  | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9

export {}
