'use strict';

let PROTO_PATH = __dirname + '/../sample.proto';

let grpc = require('@grpc/grpc-js');
let protoLoader = require('@grpc/proto-loader');
let packageDefinition = protoLoader.loadSync(
    PROTO_PATH,
    {keepCase: true,
     longs: String,
     enums: String,
     defaults: true,
     oneofs: true
    });
let sample_proto = grpc.loadPackageDefinition(packageDefinition).sample;
let client = new sample_proto.Sample('cxx:50051', grpc.credentials.createInsecure());

module.exports = grpcRequest;

function grpcRequest(param, callback) {
  client.GetSample({flag: param}, callback);
}

// grpcRequest('hogehoge');