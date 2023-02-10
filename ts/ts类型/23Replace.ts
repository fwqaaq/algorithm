/** @format */

type Replace<
  T extends string,
  K extends string,
  V extends string
> = T extends `${infer U}${K}` ? `${U}${V}` : T

type replaced = Replace<"types are fun!", "fun", "awesome"> // expected to be 'types are awesome!'

export {}
