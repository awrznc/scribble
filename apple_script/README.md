AppleScript
===

## Run

```zsh
# Command
osascript -e 'log "Hello, World!"'

# File
osascript ./src/hello_world/Main.applescript 
```

## Example

```zsh
# `ポン`という音と共に通知を出す（ビルド終了のお知らせ通知などに利用すると便利）
osascript -e 'display notification "finished!" sound name "Pop"'

# エイリアスを作成してもいいかも
# alias nofi="osascript -e 'display notification \"finished!\" sound name \"Pop\"'"
nofi
```
