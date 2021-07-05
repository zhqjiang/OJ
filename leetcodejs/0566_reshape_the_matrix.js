/**
 * @param {number[][]} mat
 * @param {number} r
 * @param {number} c
 * @return {number[][]}
 */
var matrixReshape = function (mat, r, c) {
  if (mat.length === 0) {
    return mat
  }

  if (mat[0].length === 0) {
    return mat
  }

  let m = mat.length
  let n = mat[0].length

  if (m * n !== r * c) {
    return mat
  }

  let res = []
  for (let l = 0; l < r; l++) {
    res.push([])
  }
  for (let i = 0; i < m; i++) {
    for (let j = 0; j < n; j++) {
      let value = mat[i][j]
      let idx = i * n + j
      let new_i = Math.floor(idx / c)
      res[new_i].push(value)
    }
  }
  return res
}
