const fs = require('fs');
const txt = fs.readFileSync('./test.data.txt', 'utf8');
console.log(txt.split('\n'))