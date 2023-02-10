/** @format */

interface Cat {
  type: "cat"
  breeds: "Abyssinian" | "Shorthair" | "Curl" | "Bengal"
}

interface Dog {
  type: "dog"
  breeds: "Hound" | "Brittany" | "Bulldog" | "Boxer"
  color: "brown" | "white" | "black"
}

type LookUp<T extends { type: string }, V extends T["type"]> = T extends {
  type: V
}
  ? T
  : never

type MyDogType = LookUp<Cat | Dog, "dog"> // expected to be `Dog`

export {}
