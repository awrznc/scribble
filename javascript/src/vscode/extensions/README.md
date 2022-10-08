Extensions
===

## 環境構築

```bash
docker run --rm -it -v "$(pwd):/opt/" node:18 bash
```

```bash
adduser developer
su developer
npm install
```

## 機能開発

1. `code .` でプロジェクトを開く
1. `F5` キーを押して拡張機能をコンパイル
1. `Ctrl + Shift + P` キーを押して `Hello World` コマンドが実行できたら準備完了

### 参考

https://code.visualstudio.com/api/get-started/your-first-extension
