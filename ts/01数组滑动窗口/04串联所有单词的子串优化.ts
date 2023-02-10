function findSubstring(s: string, words: string[]): number[] {
  let wordLength = words[0].length
  let wordsLength = words.length
  let complyNums: number[] = []
  //maps1记忆数组的个数
  const maps1 = new Map<string, number>()
  for (let word of words) {
    // 判断有没有子串,没有就是1,每有一次向上+1
    maps1.set(word, maps1.has(word) ? maps1.get(word) + 1 : 1)
  }
  //用i来表示第一个滑动窗口左指针
  for (let i = 0; i <= s.length - wordsLength * wordLength; i++) {
    //每循环一次same都重新更改为true
    let same = true
    //用来和maps1进行比较
    const maps2 = new Map<string, number>()
    //截取一个数组单词个数的总长度的窗口
    let wordsStr = s.substr(i, i + wordLength * words.length)
    //j表示左指针,左指针指向单词长度的滑动窗口
    for (
      let j = 0;
      j <= wordLength * words.length - wordLength;
      j += wordLength
    ) {
      let wordStr = wordsStr.substr(j, wordLength)
      //判断wordsStr滑动窗口有没有符合子串
      if (maps1.has(wordStr)) {
        maps2.set(wordStr, maps2.has(wordStr) ? maps2.get(wordStr) + 1 : 1)
      } else {
        break
      }
      //如果maps2中值大于maps1中,就代表子串多余maps1,直接返回
      if (maps2.get(wordStr) > maps1.get(wordStr)) break
    }
    //如果两个表相等,那么就符合条件
    for (let [key, value] of maps1) {
      if (maps2.get(key) !== value) {
        same = false
        break
      }
    }
    if (same) complyNums.push(i)
  }
  return complyNums
}

console.log(findSubstring("barfoothefoobarman", ["foo", "bar"]))
export {}
