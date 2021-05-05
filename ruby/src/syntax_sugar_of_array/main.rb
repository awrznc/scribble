array = ['a', 'b']
string = 'b'

case string
when *array
  puts 'allow'
else
  puts 'error'
end
# => "allow"

string = 'c'
case string
when *array
  puts 'allow'
else
  puts 'error'
end
# => "error"

