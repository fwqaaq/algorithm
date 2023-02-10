/** @format */

type TupleToNestedObject<K extends any[], V> = K extends [infer F, ...infer R]
  ? // Record<K[0], TupleToNestedObject<R, V>>
    {
      [P in F as K[0]]: TupleToNestedObject<R, V>
    }
  : V

type a = TupleToNestedObject<["a"], string> // {a: string}
type b = TupleToNestedObject<["a", "b"], number> // {a: {b: number}}
type c = TupleToNestedObject<[], boolean> // boolean. if the tuple is empty, just return the U type
export {}
