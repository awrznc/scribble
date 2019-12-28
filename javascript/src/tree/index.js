const fs = require('fs');
const path = require('path');

function tree(dir, ignores) {
  const filenames = fs.readdirSync(dir);
  filenames.forEach((filename) => {
    const fullPath = path.join(dir, filename);
    const stats = fs.statSync(fullPath);
    if (stats.isFile() && ignores.every(reg => !fullPath.match(reg)) ) {
      console.log(fullPath);
    } else if (stats.isDirectory() && ignores.every(reg => !fullPath.match(reg)) ) {
      tree(fullPath, ignores);
    }
  });
}

// 正規表現の配列を利用して、ignoreするファイル・ディレクトリを指定する
const ignores = [/sample_b/]

// それとpwd
const dir = process.cwd();

tree(dir, ignores);
// => ...\index.js
// => ...\sample_dir\sample_a
// ( sample_b は表示されない)