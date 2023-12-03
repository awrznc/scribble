Hello, World
===

SSG 対応の HelloWorld。

## Create

```bash
$ docker run --rm -p '5173:5173' -p '4173:4173' -v "$(pwd):/opt/workspace" -w /opt/workspace -it node:21 bash
$ npm create svelte@latest hello-world
Need to install the following packages:
create-svelte@5.3.2
Ok to proceed? (y) y

create-svelte version 5.3.2

┌  Welcome to SvelteKit!
│
◇  Which Svelte app template?
│  Skeleton project
│
◇  Add type checking with TypeScript?
│  No
│
◇  Select additional options (use arrow keys/space bar)
│  none
│
└  Your project is ready!

Install community-maintained integrations:
  https://github.com/svelte-add/svelte-add

Next steps:
  1: cd hello-world
  2: npm install
  3: git init && git add -A && git commit -m "Initial commit" (optional)
  4: npm run dev -- --open

To close the dev server, hit Ctrl-C

Stuck? Visit us at https://svelte.dev/chat

$ cd hello-world && npm install
$ npm i @sveltejs/adapter-static
# svelte.config.js の修正
```

## Run

```bash
# package.json の vite dev に `--host 0.0.0.0` を追加
$ npm run dev
# => http://localhost:5173/

# +layout.server.js の追加
$ npm run build
# => build/ 配下に生成される

# ビルド結果の確認
$ npm run preview
# => http://localhost:4173/
```
