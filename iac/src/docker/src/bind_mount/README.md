Bind mount
===

ホストマシン上のファイルやディレクトリをコンテナ内にマウントできる機能。


## 例

現在のディレクトリを `/opt/work` にマウントする例。

```bash
docker run \
    --mount type=bind,source="$(pwd)"/,target=/opt/work \
    --workdir /opt/work \
    --rm -it debian:11 bash
```


## Volume との違い

`--mount` の方が説明的かつ冗長とのこと。<br>
例えば、ホスト上にマウント対象がない場合はエラーになるらしい ( `--volume` の場合はディレクトリが作成される )

2025年現在は `--mount` を利用することが推奨されている。

参考: https://docs.docker.jp/storage/bind-mounts.html
