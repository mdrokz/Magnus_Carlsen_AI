const cp = require('child_process');
// const net = require('net')
const fs = require('fs');
const ffi = require('ffi')


var lib = ffi.Library('./libwrite_html', {
    'writeLinks': ['void', ['string']]
})

var page_number = Number(process.argv[2]) + 1;

var dns_server = process.argv[3];

if (!page_number) {
    console.error("ERROR: PAGE NUMBER IS REQUIRED")
    process.exit(-1)
}

console.log(page_number)

for (var i = 1; i < page_number; i++) {
    cp.exec(`wget --quiet -O - "https://chessgames.com/perl/chess.pl?page=${i}&pid=52948"`, (err, stdout, stderr) => {
        // console.log(stdout.length);
        fs.writeFile(`./html/t${i}.html`,stdout,(err) => err ? console.error(err) : null)
        // client.write(stdout, (err) => console.error(err))
        // lib.writeLinks(stdout)
    })
}

// var client = net.createConnection('./socket')
// .on('connect', ()=>{
//     console.log("Connected.");

//     for (var i = 1; i < page_number; i++) {
//         cp.exec(`wget --quiet -O - "https://chessgames.com/perl/chess.pl?page=${i}&pid=52948"`, (err, stdout, stderr) => {
//             // console.log(stdout);
//             client.write(stdout,(err) => console.error(err))
//         })
//     }
// })