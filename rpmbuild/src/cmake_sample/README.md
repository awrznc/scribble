## C Build


```bash
# ビルドディレクトリの作成
mkdir build
cd build

# ビルド
cmake ../src
cmake --build .

# 実行の確認
./sample_app

# リンカの確認
ldd sample_app
        linux-vdso.so.1 (0x00007ffc1a9f0000)
        libsample.so => not found    # not foundになってる！
        libc.so.6 => /lib64/libc.so.6 (0x00007fdf45d91000)
        /lib64/ld-linux-x86-64.so.2 (0x00007fdf46154000)

# 戻る
cd ../
```

## RPM Build

```bash
# Build
export SRC=$(pwd) && rpmbuild -bb sample.spec

# 依存関係の確認
rpm -qRp /root/rpmbuild/RPMS/x86_64/sample-1.0.0-1.x86_64.rpm
```

## Install

```bash
rpm -ivh /root/rpmbuild/RPMS/x86_64/sample-1.0.0-1.x86_64.rpm
```

## Run

```bash
/usr/local/sample/bin/sample_app
```
