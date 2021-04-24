/**
 * initialize your data structure here.
 */
var MinStack = function () {
  this.data = []
  this.min = []
}

/**
 * @param {number} val
 * @return {void}
 */
MinStack.prototype.push = function (val) {
  this.data.push(val)
  let min_size = this.min.length
  if (min_size == 0) {
    this.min.push(val)
  } else if (val <= this.min[min_size - 1]) {
    this.min.push(val)
  }
}

/**
 * @return {void}
 */
MinStack.prototype.pop = function () {
  if (this.data.length == 0) {
    return
  } else {
    let val = this.data.pop()
    let min_size = this.min.length
    if (val == this.min[min_size - 1]) {
      this.min.pop()
    }
  }
}

/**
 * @return {number}
 */
MinStack.prototype.top = function () {
  let size = this.data.length
  if (size > 0) {
    return this.data[size - 1]
  } else {
    return null
  }
}

/**
 * @return {number}
 */
MinStack.prototype.getMin = function () {
  let min_size = this.min.length
  if (min_size > 0) {
    return this.min[min_size - 1]
  } else {
    return null
  }
}

// let minStack = new MinStack()
// minStack.push(-2)
// minStack.push(0)
// minStack.push(-3)
// console.log(minStack)
// let a = minStack.getMin() // return -3
// console.log(a)
// let b = minStack.pop()
// let c = minStack.top()
// console.log(c)
// let d = minStack.getMin() // return -2
// console.log(d)
