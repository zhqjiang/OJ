function squareDigits(num) {
  var newarr = new Array()
  num
    .toString()
    .split("")
    .forEach((element) => {
      newarr.push(element * element)
    })
  return Number(newarr.join(""))
}

// Best Practice

function squareDigits2(num) {
  return Number(
    ("" + num)
      .split("")
      .map(function (val) {
        return val * val
      })
      .join("")
  )
}
