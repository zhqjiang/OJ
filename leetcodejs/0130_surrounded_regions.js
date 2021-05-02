/**
 * @param {character[][]} board
 * @return {void} Do not return anything, modify board in-place instead.
 */
var solve = function (board) {
  const m = board.length
  const n = board[0].length
  const seen = new Array(m).fill(null)
  for (let i = 0; i < m; i++) {
    seen[i] = new Array(n).fill(false)
  }

  function search(i, j) {
    if (i < 0 || i >= m || j < 0 || j >= n || seen[i][j]) {
      return
    }
    seen[i][j] = true
    if (board[i][j] == "X") {
      return
    } else {
      search(i - 1, j)
      search(i + 1, j)
      search(i, j - 1)
      search(i, j + 1)
    }
  }

  for (let i = 0; i < m; i++) {
    search(i, 0)
    search(i, n - 1)
  }

  for (let j = 0; j < n; j++) {
    search(0, j)
    search(m - 1, j)
  }

  for (let i = 0; i < m; i++) {
    for (let j = 0; j < n; j++) {
      if (!seen[i][j]) {
        board[i][j] = "X"
      }
    }
  }
}
