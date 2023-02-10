function buildArray(nums: number[]): number[] {
  let array:number[]=[]
  for(let item of nums){
    array.push(nums[item])
  }
  return array
}

console.log(buildArray([5,0,1,2,3,4]))