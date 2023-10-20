const args = process.argv.slice(2);
const version = args[0]
const notes = args[1]
const pub_date = new Date().toISOString()
const signature = args[2]
const url = args[3]
let str = `{
    "version": "${version}",
    "notes": "${notes}",
    "pub_date": "${pub_date}",
    "platforms": {
      "windows-x86_64": {
        "signature": "${signature}",
        "url": "${url}"
      }
    }
}`
//将str变量的内容写入到当前目录下的update.json文件中
const fs = require('fs');
fs.writeFile('./update.json', str, function (err) {
    if (err) {
      console.error(err);
    }
  }
);