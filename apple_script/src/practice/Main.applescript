
-- 標準出力
log "AppleScript"

-- 通知
display notification "sample"

-- 通知 (+効果音)
display notification "sample" sound name "Pop"

-- 背景の変更 (Pro)
tell application "Terminal" to set current settings of first window to settings set "Pro"

-- 背景の変更 (Basic)
tell application "Terminal" to set current settings of first window to settings set "Basic"

-- ウインドウ名の取得
tell application "Terminal" to get name of front window
