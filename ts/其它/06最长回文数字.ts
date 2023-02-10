function getConcatenation(nums: number[]): number[] {
  return [...nums, ...nums];
}

console.log(getConcatenation([2, 3, 4, 5, 6]));
