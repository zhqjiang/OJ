/**
 * @param {number} x
 * @return {number}
 */
var reverse = function (x) {
  let result =
    Math.sign(x) *
    Math.abs(x)
      .toString()
      .split("")
      .map((ele) => parseInt(ele, 10))
      .reduce((acc, ele, idx) => acc + ele * Math.pow(10, idx), 0)
  return result <= 0x7fffffff && result >= -0x80000000 ? result : 0
}
