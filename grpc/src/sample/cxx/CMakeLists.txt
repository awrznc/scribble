cmake_minimum_required(VERSION 3.5.1)
project(HelloWorld C CXX)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

# find packages
find_package(Threads REQUIRED)
set(protobuf_MODULE_COMPATIBLE TRUE)
find_package(Protobuf CONFIG REQUIRED)
message(STATUS "Using protobuf ${Protobuf_VERSION}")
find_package(gRPC CONFIG REQUIRED)
message(STATUS "Using gRPC ${gRPC_VERSION}")

# Generated sources
get_filename_component(hw_proto "../sample.proto" ABSOLUTE)
get_filename_component(hw_proto_path "${hw_proto}" PATH)
set(hw_proto_srcs "${CMAKE_CURRENT_BINARY_DIR}/sample.pb.cc")
set(hw_proto_headers "${CMAKE_CURRENT_BINARY_DIR}/sample.pb.h")
set(hw_grpc_srcs "${CMAKE_CURRENT_BINARY_DIR}/sample.grpc.pb.cc")
set(hw_grpc_headers "${CMAKE_CURRENT_BINARY_DIR}/sample.grpc.pb.h")
add_custom_command(
      OUTPUT "${hw_proto_srcs}" "${hw_proto_headers}" "${hw_grpc_srcs}" "${hw_grpc_headers}"
      COMMAND $<TARGET_FILE:protobuf::protoc>
      ARGS --grpc_out "${CMAKE_CURRENT_BINARY_DIR}"
        --cpp_out "${CMAKE_CURRENT_BINARY_DIR}"
        -I "${hw_proto_path}"
        --plugin=protoc-gen-grpc="$<TARGET_FILE:gRPC::grpc_cpp_plugin>"
        "${hw_proto}"
      DEPENDS "${hw_proto}")

# Build server
include_directories("${CMAKE_CURRENT_BINARY_DIR}")
add_executable(server "server.cxx" ${hw_proto_srcs} ${hw_grpc_srcs})
target_link_libraries(server gRPC::grpc++_reflection gRPC::grpc++ protobuf::libprotobuf)

# Build client
add_executable(client "client.cxx" ${hw_proto_srcs} ${hw_grpc_srcs})
target_link_libraries(client gRPC::grpc++_reflection gRPC::grpc++ protobuf::libprotobuf)