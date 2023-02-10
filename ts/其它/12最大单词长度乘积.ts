function maxProduct(words: string[]) {
  let noRepeatArray: string[] = [];
  const noRepeatString = new Set(words);
  noRepeatArray.push(...noRepeatString);
  console.log(noRepeatArray);
}

maxProduct(["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]);
