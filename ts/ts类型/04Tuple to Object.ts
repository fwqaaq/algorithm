/** @format */

const tuple = ["tesla", "model 3", "model X", "model Y"] as const

type TupleToObject<K extends readonly any[]> = {
  [P in K[number]]: P
}
type result = TupleToObject<typeof tuple> // expected { tesla: 'tesla', 'model 3': 'model 3', 'model X': 'model X', 'model Y': 'model Y'}

export {}
