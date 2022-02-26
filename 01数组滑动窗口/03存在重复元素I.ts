function containsDuplicate(nums: number[]): boolean {
  //ES6
  //return new Set(nums).size !== nums.length ?? false
  //ES5
  return (
    nums.filter(function (item, index, arr) {
      return arr.indexOf(item) === index
    }).length !== nums.length ?? false
  )
}

console.log(containsDuplicate([1, 2, 3, 1]))
