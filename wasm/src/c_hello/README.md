## Build


```bash
# https://github.com/emscripten-core/emscripten/issues/6351#issuecomment-41403021
# -03 じゃなくて -O3 じゃね。。？
emcc main.c -o index.wasm -O3

deno run --allow-read app.js
```
