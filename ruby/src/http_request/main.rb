require 'uri'
require 'json'
url = "http://xxx.xxx.xxx.xxx/sample.json"

require 'net/http'
response = Net::HTTP.get(URI.parse( URI.encode(url) ))

require 'open-uri'
response = JSON.parse(URI.open(url).read)
