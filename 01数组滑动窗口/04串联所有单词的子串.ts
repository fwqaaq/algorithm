function findSubstring(s: string, words: string[]) {
  let wordLength = words[0].length
  let wordsLength = words.length
  let complyNums: number[] = []
  for (let left = 0; left < s.length; left++) {
    //使用深拷贝,每次循环更新对比指针
    let wordsArray = [...words]
    // 初始化一个右指针
    let right = 0
    //右指针必须连续wordsLength次移动并且符合条件
    while (right < wordsLength) {
      //每次移动,更新单词
      let word = s.substr(left + right * wordLength, wordLength)
      //更新数组中单词的索引,并删掉有的单词,如果都删完,则符合条件
      let wordIndex = wordsArray.indexOf(word)
      if (wordIndex !== -1) {
        wordsArray.splice(wordIndex, 1)
      }
      right++
    }
    if (wordsArray.length === 0) {
      complyNums.push(left)
    }
  }
  return complyNums
}

console.log(findSubstring("barfoothefoobarman", ["foo", "bar"]))
