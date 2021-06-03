/**
 * @param {string} s
 * @return {number}
 */
var numDecodings = function (s) {
  let counts_two = 0
  let counts_one = s[0] != "0" ? 1 : 0

  s.split("")
    .map((x) => parseInt(x, 10))
    .forEach((ele, idx) => {
      if (idx > 0) {
        let tmp = counts_one
        counts_one = ele > 0 ? counts_two + counts_one : 0
        if (s[idx - 1] == 1 || (s[idx - 1] == 2 && ele <= 6)) {
          counts_two = tmp
        } else {
          counts_two = 0
        }
      }
    })

  return counts_two + counts_one
}
