const path = require('path');

module.exports = {
    // モード
    mode: "development",

    // エントリーポイント
    entry: './src/index.js',

    // 出力するファイル
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bundle.js'
    },
}
