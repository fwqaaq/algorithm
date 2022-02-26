function twoSum(nums: number[], target: number): number[] | null {
  if (nums.length < 2) {
    return null;
  }
  let indexArry: number[] = [];
  for (let value of nums) {
    let otherValue = target - value;
    let otherIndex = nums.indexOf(otherValue);
    if (otherIndex > 0) {
      let index = nums.indexOf(value);
      indexArry = [index, otherIndex];
    }
  }
  return indexArry;
}
console.log(twoSum([1, 2, 3], 4));
export {};
