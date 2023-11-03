//读取 ./src-tauri/tauri.conf.json的内容，打印"package"属性的"version"属性
const { readFileSync } = require('fs')

let fileContext = readFileSync('./src-tauri/tauri.conf.json', 'utf-8');
let version = JSON.parse(fileContext).package.version;
console.log(version);