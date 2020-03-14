
## Build

### 自作コマンドの作成

なんでもいいけど、今回はgccで作成。

```bash
gcc -o ./src/hello ./src/main.c
```

### RPM Build

```bash
export SRC=$(pwd) && rpmbuild -bb hello.spec
```

### install

```bash
rpm -ivh /root/rpmbuild/RPMS/x86_64/hello-1.0.0-1.x86_64.rpm
```

### memo

```bash
# パッケージファイル情報の詳細表示。(infomation)
rpm -qip {{パッケージファイル名}}

# 該当パッケージでインストールされるファイル一覧の表示。(list)
rpm -qlp {{パッケージファイル名}}

# パッケージが依存しているファイルの表示。(require)
rpm -qRp {{パッケージファイル名}}

# インストールしたパッケージの詳細
rpm -qi {{パッケージ名}}
```

* %{xxxx}のMacro部分はrpm --evalで確認できる。