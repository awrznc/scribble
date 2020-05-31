require 'webrick'
require 'pg'

server = WEBrick::HTTPServer.new(
  :BindAddres => '0.0.0.0',
  :Port => 9292,
  :HTTPVersion => WEBrick::HTTPVersion.new('1.1')
)


# sample TABLE の内容を表示する
def print_sample_table()

  # 接続のための設定
  database_config = {
    host: "postgres",
    user: "postgres",
    password: "postgres",
    # dbname: "sample",     # CREATE DATABASE などをしたい場合は省略可
    port: "5432"
  }

  # 接続・取得・表示
  postgres = PG::connect(database_config)
  results = postgres.exec("SELECT * FROM sample")
  results.each do |tuple|
    puts "id: #{tuple['id']}, text: #{tuple['value']}"
  end

  # p results = postgres.exec("DROP TABLE sample")
  # p results = postgres.exec("DROP DATABASE foo")
  # p results = postgres.exec("CREATE DATABASE foo")
  # p results = postgres.exec("CREATE TABLE sample ( id INT PRIMARY KEY, value TEXT )")
  # p results = postgres.exec("INSERT INTO sample values ( 1, 'aaa' )")

  postgres.finish
end


server.mount_proc('/') do |req, res|

  # request
  puts "========== #{Time.new} =========="
  puts req.request_line
  puts ''
  puts req.raw_header
  puts "\n#{req.body}" if req.body

  # response
  res.status = 200
  res['Content-Type'] = 'text/html'
  res.body = "hello"
  print_sample_table()
end


server.start
