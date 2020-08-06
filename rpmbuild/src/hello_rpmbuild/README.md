# Run

```bash
# rpmbuild --define "_topdir {{build_dir_path}}" -bb ./main.spec
rpmbuild --define "_topdir $(pwd)/hello" -bb ./hello/SPECS/hello.spec
```

# Install

```
rpm -ivh ./hello/RPMS/x86_64/hello-1-1.el8.x86_64.rpm
```

# memo

* ビルドディレクトリ名はパッケージ名に合わせること（合わせないとうまくインストールできない）
