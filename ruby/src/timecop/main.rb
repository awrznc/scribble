require 'date'
require 'timecop'

p Time.now
# => 2019-04-17 09:36:19 +0900

datetime = Time.parse('2019-04-16 18:00:00 +0900')
p Timecop.freeze(datetime)
# => 2019-04-16 18:00:00 +0900

p Time.now
# => 2019-04-16 18:00:00 +0900
