function revrot(str, sz) {
  if (sz <= 0 || sz === undefined || sz > str.length) {
    return ""
  }
  let strIsOdd = false
  str.split("").forEach((ele) => {
    if (
      ele === "1" ||
      ele === "3" ||
      ele === "5" ||
      ele === "7" ||
      ele === "9"
    ) {
      strIsOdd = !strIsOdd
    }
  })
}

function isOdd(ele) {
  return
}
