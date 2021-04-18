/*
deno run --allow-write gen.js
*/
let cost_up_litmit = 9999
let n = 12
let cap = Math.round(cost_up_litmit * n / 2)

let values = ""
for (let i = 0; i < n; i++) {
    let num = Math.floor(Math.random() * 999)
    values += `${num} `
}
values.trim()
values += "\n"

let costs = ""
for (let i = 0; i < n; i++) {
    let num = Math.floor(Math.random() * cost_up_litmit)
    costs += `${num} `
}
costs.trim()
costs += "\n"

let final = `1\n${n} ${cap}\n` + values + costs

const write = Deno.writeTextFile("./test.txt", final);