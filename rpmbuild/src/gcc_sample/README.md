
## Build

### 自作コマンドの作成

なんでもいいけど、今回はgccで作成。

```bash
gcc -o ./src/hello ./src/main.c
# gcc -o ./hello/SOURCES/hello ./src/main.c
tar czvf ./hello/SOURCES/hello-1.0.tar.gz ./src/hello
```

### RPM Build

```bash
# rpmbuild --define "_topdir {{build_dir_path}}" -bb ./*.spec
rpmbuild --define "_topdir $(pwd)/hello" -bb hello/SPECS/sample.spec
```

### install

```bash
rpm -ivh ./hello/RPMS/x86_64/hello-1-1.el8.x86_64.rpm
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