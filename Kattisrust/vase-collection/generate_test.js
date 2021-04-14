const fs = require("fs");

let number = 50;
let s = "1\n" + number.toString() + "\n";

for (let i = 0; i < number; i++) {
    s += (Math.floor(Math.random() * 20) + 1).toString() + ' ' +
        (Math.floor(Math.random() * 20) + 1).toString() + '\n';
}

fs.writeFile('in.txt', s, function (err, data) {
    if (err) {
        return console.log(err);
    }
    console.log(data);
})
