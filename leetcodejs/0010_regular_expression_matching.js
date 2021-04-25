/**
 * @param {string} s
 * @param {string} p
 * @return {boolean}
 */
var isMatch = function (s, p) {
  let ma = [...new Array(s.length + 1)].map(() =>
    [...new Array(p.length + 1)].map(() => false)
  )

  ma[0][0] = true
  for (let i = 0; i < p.length; i += 1) {
    if (p[i] == "*" && ma[0][i - 1]) {
      ma[0][i + 1] = true
    }
  }

  for (let i = 0; i < s.length; i += 1) {
    for (let j = 0; j < p.length; j += 1) {
      if (s[i] == p[j] || p[j] == ".") {
        ma[i + 1][j + 1] = ma[i][j]
        continue
      }

      if (p[j] == "*") {
        if (p[j - 1] != s[i] && p[j - 1] != ".") {
          ma[i + 1][j + 1] = ma[i + 1][j - 1]
        } else {
          ma[i + 1][j + 1] = ma[i + 1][j] || ma[i][j + 1] || ma[i + 1][j - 1]
        }
      }
    }
  }

  return ma[s.length][p.length]
}

console.log(isMatch("aa", "a") == false)
console.log(isMatch("aa", "a*") == true)
console.log(isMatch("ab", ".*") == true)
console.log(isMatch("", "c") == false)
console.log(isMatch("", ".") == false)
console.log(isMatch("c", "") == false)
console.log(isMatch("aaaadb", "ba*b") == false)
console.log(isMatch("dab", "b.*b") == false)
console.log(isMatch("aaaad", ".*d") == true)
console.log(isMatch("bad", ".*d") == true)
console.log(isMatch("", "") == true)
console.log(isMatch("bab", "ba*b") == true)
console.log(isMatch("bb", "ba*b") == true)
console.log(isMatch("baaaab", "ba*b") == true)
console.log(isMatch("bab", "b.*b") == true)
console.log(isMatch("bb", "b.*b") == true)
console.log(isMatch("baaaadb", "ba*b") == false)
console.log(isMatch("bb", "ba*b") == true)
console.log(isMatch("abcd", "d*") == false)
console.log(isMatch("aab", "c*a*b") == true)
