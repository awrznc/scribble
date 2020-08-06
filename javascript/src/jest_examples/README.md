## run

```bash

# テスト実行
$ npx jest

# カバレッジ情報の出力
$ npx jest --coverage

# describeやitなどのテストケースの文言も出力
$ npx jest --verbose

# describeとファイルを指定して実行（npmで実行する場合）
$ npm test -- --testNamePattern="#httpPost" __tests__/src/axios.spec.js
```