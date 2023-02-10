/** @format */

interface ITest {
  title: string
  description: string
  completed: boolean
}

//* PICK
type MyPick<K, V extends keyof K> = {
  [P in V]: K[P]
}
type PickPreview = MyPick<ITest, "title" | "completed">

const t: PickPreview = {
  title: "Clean room",
  completed: false,
}

export {}
