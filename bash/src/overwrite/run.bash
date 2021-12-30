# cron.bash をバックグラウンドプロセスとして起動
bash -x cron.bash &
sleep 1

# 実行中にファイルを上書き
cp ./cron_after.bash ./cron.bash
# => 変数は上書きされずechoではなくbannarが実行される

# バックグラウンドプロセスが終了するまで待機してから終了する
wait $!
