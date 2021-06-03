function lastSurvivors(arr, nums) {
  const arr2d = arr.map((str) => str.split(""))
  nums.forEach((count, index) => {
    trimArr(index, count, arr2d)
  })

  return arr2d.map((arr) => arr.filter((ele) => ele != " ").join("")).join("")
}

function trimArr(index, count, arr2d) {
  let n = arr2d.length - 1

  while (n >= 0 && count >= 1) {
    let rowArr = arr2d[n]

    // if this row is not long enough
    // move to previous row
    if (rowArr.length <= index) {
      n -= 1
      continue
    }

    // if the index-th element of this row is blank
    // move to previous row
    if (rowArr[index] === " ") {
      n -= 1
      continue
    }

    // replace index-th element with blank character
    rowArr[index] = " "
    count -= 1
    n -= 1
  }
}
