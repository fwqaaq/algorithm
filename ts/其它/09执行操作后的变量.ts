function finalValueAfterOperations(operations: string[]): number {
  let index = 0;
  for (let value of operations) {
    if (value === "++X" || value === "X++") {
      index++;
    }
    if (value === "--X" || value === "X--") {
      index--;
    }
  }
  console.log(index);
  return index;
}

finalValueAfterOperations(["++X", "++X", "X++"]);

export {};
