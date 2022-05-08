/** @format */

type ReplaceKeys<T, K extends string, V> = V extends V
  ? {
      [P in keyof T]: P extends K ? (P extends keyof V ? V[P] : never) : T[P]
    }
  : never

type NodeA = {
  type: "A"
  name: string
  flag: number
}

type NodeB = {
  type: "B"
  id: number
  flag: number
}

type NodeC = {
  type: "C"
  name: string
  flag: number
}

type Nodes = NodeA | NodeB | NodeC

type ReplacedNodes = ReplaceKeys<
  Nodes,
  "name" | "flag",
  { name: number; flag: string }
> // {type: 'A', name: number, flag: string} | {type: 'B', id: number, flag: string} | {type: 'C', name: number, flag: string} // would replace name from string to number, replace flag from number to string.

type ReplacedNotExistKeys = ReplaceKeys<Nodes, "name", { aa: number }>
// {type: 'A', name: never, flag: number} | NodeB | {type: 'C', name: never, flag: number} // would replace name to never

export {}
