/*
 * @lc app=leetcode.cn id=1002 lang=typescript
 *
 * [1002] 查找共用字符
 */

// @lc code=start
function commonChars(words: string[]) {
  let str: string = ""
  //设置一个用字母组成的map字典
  let map = new Map<string, number>()
  //给所有map设置初始值为0
  let wordInitial: [string, number][] = words[0]
    .split("")
    .map((item) => [item, 0])
  //如果有重复字母,就把重复字母的数量加1
  for (let word of words[0]) {
    map.set(word, map.has(word) ? map.get(word)! + 1 : 1)
  }
  for (let i = 1; i < words.length; i++) {
    const mapWord = new Map<string, number>(wordInitial)
    for (let j = 0; j < words[i].length; j++) {
      if (!map.has(words[i][j])) continue
      //mapWord中的字母的个数不能高于当前map的个数,多于则不能添加
      if (map.get(words[i][j])! > mapWord.get(words[i][j])!) {
        mapWord.set(
          words[i][j],
          mapWord.has(words[i][j]) ? mapWord.get(words[i][j])! + 1 : 1
        )
      }
    }
    //每次重新初始化map
    map = mapWord
  }
  for (let [key, value] of map) {
    str += key.repeat(value)
  }

  return str.split("")
}
// @lc code=end
console.log(commonChars(["bella", "label", "roller"]))
