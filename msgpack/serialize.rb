raise "Empty ARGV." if ARGV[0].nil?

require 'msgpack'

data = [ [12, 34], [56, 78] ]

File.open(ARGV[0], "w") do | file |
  file.print( data.to_msgpack )
end

# # Load msgpack file
# # File.open("./data.mpac") do |file|
# File.open(ARGV[0]) do |file|
#   MessagePack::Unpacker.new(file).each do |obj|
#     puts obj
#   end
# end

puts "[INFO] Wrote Message Pack data: #{ARGV[0]}"
