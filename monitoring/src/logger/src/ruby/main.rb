# 実行: docker run --rm -v "$(pwd):/opt/workspace" -w /opt/workspace ruby:3.2 bash -c 'ruby ./main.rb'
# 参考: https://docs.ruby-lang.org/ja/latest/library/logger.html

require 'logger'

logger = Logger.new(STDOUT)

# ログレベルの指定
# 指定されたレベルよりも低いログは記録されない
# 実行環境ごとに出し分けると便利
# 例: WARNよりも低いログは記録しない
logger.level = Logger::WARN

# 出力
logger.debug('debug!') # 出力されない
logger.error('error!') # 出力される
# => E, [2023-04-01T00:00:00.000000 #1] ERROR -- : error!
