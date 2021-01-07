#include <iostream>
#include <memory>
#include <string>

#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>
#include <grpcpp/ext/proto_server_reflection_plugin.h>

#include "sample.grpc.pb.h"

using grpc::Server;
using grpc::ServerBuilder;
using grpc::ServerContext;
using grpc::Status;
using sample::Hoge;
using sample::Piyo;
using sample::Sample;

class SampleServiceImpl final : public Sample::Service {
    Status GetSample(
        ServerContext* context,
        const Hoge* request,
        Piyo* reply
    ) override {
        std::cout << "通った" << std::endl;
        std::string prefix("Hello ");
        reply->set_name(prefix + request->flag());
        reply->set_kind(prefix + request->flag());
        return Status::OK;
    }
};

void RunServer() {
    std::string server_address("0.0.0.0:50051");
    SampleServiceImpl service;

    grpc::EnableDefaultHealthCheckService(true);
    grpc::reflection::InitProtoReflectionServerBuilderPlugin();
    ServerBuilder builder;
    builder.AddListeningPort(server_address, grpc::InsecureServerCredentials());
    builder.RegisterService(&service);
    std::unique_ptr<Server> server(builder.BuildAndStart());
    std::cout << "Server listening on " << server_address << std::endl;
    server->Wait();
}

int main(int argc, char** argv) {
    RunServer();

    return 0;
}