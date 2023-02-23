function prompt() {
  "PS " + (Split-Path (Get-Location) -Leaf) + " " + (Get-Date -Format "yyyy/MM/dd HH:mm:ss > ")
}
