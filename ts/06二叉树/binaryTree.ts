/** @format */

import type { binaryTrees as nodeTree } from './nodeTree.js'
import { binaryTrees } from './nodeTree.js'

export const root: nodeTree<number> = new binaryTrees<number>(10)
root.left = new binaryTrees<number>(5)
root.right = new binaryTrees<number>(15)

console.log(root)
