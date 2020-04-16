# Bash

```

＿人人人＿
＞今日は＜
￣Y^Y^Y￣
　 _n
　( ｜　 ハ_ハ
　 ＼＼ ( ‘-^　) 
　　 ＼￣￣　 ) 
　　　 ７　　/ 

＿人人人人人人＿
＞  Shell芸  ＜
￣Y^Y^Y^Y^Y^Y￣
　ハ_ハ
（ ‘-^　)　　n
￣　　 ＼　( E)
７　　/＼ヽ/ /

```


## Run

```bash

# 実行
bash [file path]

# 実行ログも表示
bash -x [file path]

```

## Memo

### Shebang ( `#!` )

よくみかけるファイル頭の `#!/bin/sh` とか。

* `#!` で始まる行のことを `Shebang` ( シェバン ) という。Hash(#) と Bang(!)をくっつけた HashBang の省略形だったりする。
* bashとして動かすなら `#!/bin/bash` を指定した方がよい。記述しないと、別の種類の ShellScript などで実行される場合があるので注意（もしくはエラーが発生する）
