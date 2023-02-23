Custom-Prompt
===

PowerShellのプロンプト文字列をカスタマイズするためのメモ。

今回の例では以下のExampleのような状態にする。

```ps1
# Default
PS <Current-Full-Path> >

# Example
PS <Current-Dir-Name> >
```

手順は以下の通り。

```ps1
# mkdir -p
$TargetDirPath = "$HOME\Documents\WindowsPowerShell"
New-Item $TargetDirPath -ItemType Directory -ErrorAction SilentlyContinue

# 内容の取得
$ProfileContent = ([String](Get-Content -Path ".\Profile.ps1" -Raw))

# ファイルを作成しながら追記
$TargetFilePath = "$TargetDirPath\Microsoft.PowerShell_profile.ps1"
Write-Output $ProfileContent | Add-Content $TargetFilePath -Encoding Default
```

あとはスクリプトが実行できるように管理者権限のPowershellから権限を変えてあげればOK。

https://learn.microsoft.com/ja-jp/powershell/module/microsoft.powershell.core/about/about_execution_policies?view=powershell-7.3

例えば以下のようなコマンドで変更できる。

```ps1
Set-ExecutionPolicy RemoteSigned
```

権限を変更したくない場合は `Profile.ps1` の内容を端末に毎回打ち込んで実行すればOK。
