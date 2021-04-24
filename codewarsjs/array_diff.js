/**
 *
 * @param {number[]} a
 * @param {number[]} b
 * @returns {number[]}
 */
function arrayDiff(a, b) {
  return a.filter((ele) => b.indexOf(ele) == -1)
}
