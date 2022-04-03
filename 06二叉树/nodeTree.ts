/** @format */

export class binaryTrees<T> {
  public key: T
  public left: binaryTrees<T> | null
  public right: binaryTrees<T> | null
  constructor(
    key: T,
    left?: binaryTrees<T> | null,
    right?: binaryTrees<T> | null
  ) {
    this.key = key
    this.left = left === undefined ? null : left
    this.right = right === undefined ? null : right
  }
}
