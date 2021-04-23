/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function (nums, target) {
  let m = new Map()

  let i, j
  nums.every((ele, idx) => {
    if (m.has(target - ele)) {
      i = m.get(target - ele)
      j = idx
      return false
    } else {
      m.set(ele, idx)
      return true
    }
  })

  if (i < j) {
    return [i, j]
  } else {
    return [j, i]
  }
}
