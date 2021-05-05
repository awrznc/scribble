require 'socket'

server = TCPServer.new 2000

loop do
  client = server.accept
  headers = []
  while header = client.gets
    break if header.chomp.empty?
    headers << header.chomp
  end

  # Referer
  puts headers[12]

  client.puts "HTTP/1.0 200 OK"
  client.puts "Content-Type: application/json"
  client.puts
  client.puts '{ "text": "sample" }'
  client.close
end
