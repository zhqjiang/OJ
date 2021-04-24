var findMissing = function (list) {
  let len = list.length
  let b_diff = list[1] - list[0]
  let e_diff = list[len - 1] - list[len - 2]
  let diff = Math.abs(b_diff) >= Math.abs(e_diff) ? e_diff : b_diff

  for (let i = 0; i < len; i += 1) {
    let should_be = list[0] + diff * i
    if (list[i] !== should_be) {
      return should_be
    }
  }
}
