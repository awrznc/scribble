Ninja
===

既存のビルドシステムの高速化を目的に作成されたビルドシステム。

https://ninja-build.org/manual.html


## Run

```bash
$ ninja -f ./src/hello_world/build.ninja
[1/1] echo 'Hello, World!'
Hello, World!
```


### Build and Clean

```bash
$ ninja -f ./src/practice/build.ninja
[1/3] bash -c "echo 'Hello', 'World'! >& hello.txt"
[2/3] bash -c "echo 'Hello', 'Ninja'! >& hello2.txt"
[3/3] cat hello.txt hello2.txt
Hello, World!
Hello, Ninja!

$ ninja -f ./src/practice/build.ninja -t clean
Cleaning... 2 files.
```
