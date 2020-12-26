require 'uri'
require 'json'

# エンコードが必要な文字がある場合は以下を利用する
# URI.encode_www_form_component("日本語")
url = "http://xxx.xxx.xxx.xxx/sample.json"

require 'open-uri'
response = JSON.parse(URI.open(url).read)

require 'net/http'

# get
response = Net::HTTP.get(URI.parse( url) )

# post
response = Net::HTTP.post_form(URI.parse( url ),{'sample' => 'text'})
