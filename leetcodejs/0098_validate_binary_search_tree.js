function TreeNode(val, left, right) {
  this.val = val === undefined ? 0 : val
  this.left = left === undefined ? null : left
  this.right = right === undefined ? null : right
}

let ele2 = new TreeNode(1, undefined, undefined)
let ele4 = new TreeNode(2, undefined, undefined)
let ele5 = new TreeNode(7, undefined, undefined)
let ele3 = new TreeNode(5, ele4, ele5)
let ele1 = new TreeNode(3, ele2, ele3)

function dfs(root, lower_limit, upper_limit) {
  if (!root) {
    return true
  }

  let res = true
  if (upper_limit) {
    res = res && root.val < upper_limit
  }
  if (lower_limit) {
    res = res && root.val > lower_limit
  }

  if (root.left) {
    res = res && root.val > root.left.val
    res = res && dfs(root.left, lower_limit, root.val)
  }

  if (root.right) {
    res = res && root.val < root.right.val
    res = res && dfs(root.right, root.val, upper_limit)
  }

  return res
}

var isValidBST = function (root) {
  return dfs(root, null, null)
}

console.log(isValidBST(ele1))
