function longestCommonPrefix(strs: string[]): string {
  let flag = strs[0];
  while (
    strs.some((value) => {
      return value.slice(0, flag.length) !== flag;
    })
  ) {
    flag = flag.slice(0, flag.length - 1);
  }
  console.log(flag);
  return flag;
}

longestCommonPrefix(["flower", "flow", "flight"]);
