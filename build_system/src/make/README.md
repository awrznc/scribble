Make
===

メジャーなビルドシステム。

https://www.gnu.org/software/make/manual/make.html


## Run

```bash
$ make -f ./src/hello_world/Makefile
echo 'Hello, World!'
Hello, World!
```


### Build

```bash
$ make -f ./src/practice/Makefile
bash -c "echo 'Hello', 'World'! >& ./hello.txt"
bash -c "echo 'Hello', 'Ninja'! >& ./hello2.txt"
cat hello.txt hello2.txt
Hello, World!
Hello, Ninja!
```
