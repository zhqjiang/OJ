/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var threeSum = function (nums) {
  let n = nums.length
  if (n < 3) {
    return []
  }

  nums.sort(function (a, b) {
    return a - b
  })

  let result = []

  for (let i = 0; i < n - 2; i++) {
    if (i == 0 || nums[i] != nums[i - 1]) {
      let left = i + 1
      let right = n - 1

      while (left < right) {
        let sum = nums[i] + nums[left] + nums[right]
        if (sum == 0) {
          console.log(nums[i])
          result.push([nums[i], nums[left], nums[right]])
          while (left < right && nums[left] == nums[left + 1]) {
            left += 1
          }
          while (left < right && nums[right] == nums[right - 1]) {
            right -= 1
          }
          left += 1
          right -= 1
        } else if (sum < 0) {
          left += 1
        } else {
          right -= 1
        }
      }
    }
  }

  return result
}

let nums = [-1, 0, 1, 2, -1, -4]
console.log(threeSum(nums))
