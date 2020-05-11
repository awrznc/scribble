url = "http://xxx.xxx.xxx.xxx/sample.json"
response = Net::HTTP.get(URI.parse( URI.encode(url) ))
