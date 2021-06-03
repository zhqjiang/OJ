function validBraces(braces) {
  function pair(a, b) {
    if (a === "(" && b === ")") return true
    if (a === "[" && b === "]") return true
    if (a === "{" && b === "}") return true
    return false
  }
  const arr = []
  for (let i = 0; i < braces.length; i += 1) {
    const ele = braces[i]
    const lastEle = arr.length > 0 ? arr[arr.length - 1] : false

    if (lastEle && pair(lastEle, ele)) {
      arr.pop()
    } else {
      arr.push(ele)
    }
  }

  return arr.length === 0
}

console.log(validBraces("[]"))
console.log(validBraces("[]()"))
console.log(validBraces("[()]"))
console.log(validBraces("([]{})"))
