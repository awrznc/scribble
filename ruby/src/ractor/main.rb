# Ractor:
# スレッドセーフな並列実行を提供する

# 初期化
ractor = Ractor.new { puts "Hi!" }

# 実行
ractor.take
# => Hi!

puts "========"

# push型
["a", "b", "c"].map do |v|
  ractor = Ractor.new do
    # このブロックの中が実行対象
    # 渡した値はreceiveに格納される
    puts receive
    # Ractor.receiveでもOK
    # puts Ractor.receive
  end
  # sendメソッドで値を渡す
  ractor.send(v)
  ractor
end.each(&:take)
# =>
# a
# c
# b

puts "========"

["a", "b", "c"].map do |v|
  # 初期化時に値を渡すことも可能
  # 配列の中身に対して実行したい場合はこちらの方が楽かも
  ractor = Ractor.new v do |arg|
    puts arg
  end
  ractor
end.each(&:take)

puts "========"

# pull型
["a", "b", "c"].map do |v|
  ractor = Ractor.new v do |arg|
    Ractor.yield arg
  end
  taker = Ractor.new ractor do |r|
    puts r.take
  end
  taker
end.each(&:take)
