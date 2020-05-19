## GCC Build

```bash
# 動的ライブラリの生成
gcc -shared -fPIC ./src/lib/sample.c -o libsample.so

# 実行ファイルの生成
gcc -o sample src/main.c -L. -lsample

# （静的リンクの場合）
# gcc --static -o sample src/main.c -lsample

# 共有ライブラリパスの指定して実行（確認）
LD_LIBRARY_PATH=./ ./sample
```

## RPM Build

```bash
export SRC=$(pwd) && rpmbuild -bb sample.spec
```


## Install

```bash
rpm -ivh /root/rpmbuild/RPMS/x86_64/sample-1.0.0-1.x86_64.rpm
```

## Run

```bash
/usr/local/sample/bin/sample
```
