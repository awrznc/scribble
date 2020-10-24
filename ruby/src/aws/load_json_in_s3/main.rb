bucket = 'hoge-bucket.com'
item = 'fuga/piyo.json'
json_mock = '{ "sample": "sample" }'
stub_responses = {
  list_buckets: { buckets: [ { name: bucket } ] },
  get_object: { body: json_mock }
}

require 'aws-sdk-s3'

# # スタブを利用しない場合はこちらを利用する
# s3_client = Aws::S3::Client.new

# スタブを利用する場合はこちら
s3_client = Aws::S3::Client.new( stub_responses: stub_responses )
s3_object = s3_client.get_object( bucket: bucket, key: item)
json_string = s3_object.body.read

# # print buckets
# s3_client.list_buckets.buckets.map(&:name)

# print hash object
require 'json'
p json_hash = JSON.parse(json_string)
