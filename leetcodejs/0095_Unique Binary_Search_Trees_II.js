/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {number} n
 * @return {TreeNode[]}
 */
function generateTreesFromArr(nodes) {
  if (nodes.length === 0) {
    return [null]
  }
  let res = []
  for (let node of nodes) {
    const root = node
    const leftAllTrees = generateTreesFromArr(nodes.filter((n) => n < root))
    const rightAllTrees = generateTreesFromArr(nodes.filter((n) => n > root))
    for (const leftTree of leftAllTrees) {
      for (const rightTree of rightAllTrees) {
        res.push(new TreeNode(root, leftTree, rightTree))
      }
    }
  }
  return res
}

var generateTrees = function (n) {
  const nodes = Array.from(Array(n), (_, i) => i + 1)
  return generateTreesFromArr(nodes)
}
