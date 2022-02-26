function plusOne(digits: number[]): number[] {
  let sum = 0;
  digits.reverse();
  for (let i = 0; i < digits.length; i++) {
    sum = sum + digits[i] * 10 ** i;
  }
  sum++;
  let array: number[] = [];
  for (let value of sum.toString().split("")) {
    array.push(Number(value));
  }
  console.log(array);
  return array;
}

plusOne([9, 9]);
