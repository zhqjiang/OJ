/**
 * @param {string} s
 * @param {string} p
 * @return {boolean}
 */
var isMatch = function (s, p) {
  let asterisk = -1
  let s_cur = 0
  let p_cur = 0
  let match = 0

  while (s_cur < s.length) {
    if (p_cur < p.length && (s[s_cur] == p[p_cur] || p[p_cur] == "?")) {
      s_cur += 1
      p_cur += 1
    } else if (p_cur < p.length && p[p_cur] == "*") {
      match = s_cur
      asterisk = p_cur
      p_cur += 1
    } else if (asterisk != -1) {
      p_cur = asterisk + 1
      match += 1
      s_cur = match
    } else {
      return false
    }
  }

  while (p_cur < p.length && p[p_cur] == "*") {
    p_cur += 1
  }

  if (p_cur == p.length) {
    return true
  } else {
    return false
  }
}
