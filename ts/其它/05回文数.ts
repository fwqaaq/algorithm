function isPalindrome(x: number): boolean {
  if (x < 0) {
    return false;
  }
  let num = x;
  let n = 0;
  while (num !== 0) {
    n = n * 10 + (num % 10);
    console.log(n);
    num = Math.floor(num / 10);
    console.log(num);
  }
  return n === x;
}
/* 
let num=160.28
parseInt(num); //160 返回由字符串转换得到的整数,也适用于浮点数取整
Math.ceil(num); //161 返回大于等于其数字参数的最小整数
Math.floor(num);//160 返回小于等于其数值参数的最大整数
Math.round(num);//160 返回与给出的数值表达式最接近的整数,就是传说中的四舍五入 */
console.log(isPalindrome(0));
