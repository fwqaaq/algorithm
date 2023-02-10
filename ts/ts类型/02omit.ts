/** @format */
interface ITest {
  title: string
  description: string
  completed: boolean
}
//Omit
type MyOmit<K, V extends keyof K> = {
  [O in keyof K as O extends V ? never : O]: K[O]
}

type OmitPreview = MyOmit<ITest, "description" | "title">

const t2: OmitPreview = {
  completed: false,
}

type len<K extends any[]> = K[number]

type t = len<["string", 2]>
export {}
