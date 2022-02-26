function countSegments(s: string): number {
  s = s.trim();
  console.log(s.length);
  if (!!s.length) {
    console.log(0);
    return 0;
  }
  let count = 1;
  for (let item of s) {
    if (item === " ") {
      count++;
    }
  }
  return count;
}

countSegments("      ");
