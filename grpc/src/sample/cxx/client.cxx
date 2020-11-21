#include <iostream>
#include <memory>
#include <string>

#include <grpcpp/grpcpp.h>

#include "sample.grpc.pb.h"

using grpc::Channel;
using grpc::ClientContext;
using grpc::Status;
using sample::Hoge;
using sample::Piyo;
using sample::Sample;

class SampleClient {
public:
SampleClient(std::shared_ptr<Channel> channel)
    : stub_(Sample::NewStub(channel)) {}

std::string GetSample(const std::string& user) {
    Hoge request;
    request.set_flag(user);

    Piyo reply;
    ClientContext context;

    Status status = stub_->GetSample(&context, request, &reply);

    if (status.ok()) {
        return reply.name();
    } else {
        std::cout << status.error_code() << ": " << status.error_message()
                    << std::endl;
        return "RPC failed";
    }
}

private:
    std::unique_ptr<Sample::Stub> stub_;
};

int main(int argc, char** argv) {
    std::string target_str;
    std::string arg_str("--target");
    if (argc > 1) {
        std::string arg_val = argv[1];
        size_t start_pos = arg_val.find(arg_str);
        if (start_pos != std::string::npos) {
            start_pos += arg_str.size();
            if (arg_val[start_pos] == '=') {
                target_str = arg_val.substr(start_pos + 1);
            } else {
                std::cout << "The only correct argument syntax is --target=" << std::endl;
                return 0;
            }
        } else {
            std::cout << "The only acceptable argument is --target=" << std::endl;
            return 0;
        }
    } else {
        target_str = "0.0.0.0:50051";
    }
    SampleClient client(grpc::CreateChannel(target_str, grpc::InsecureChannelCredentials()));
    std::string user("sample");
    std::string reply = client.GetSample(user);
    std::cout << "Sample received: " << reply << std::endl;

    return 0;
}