package main

import (
    "log"
    "net"
    protobuf 	"./sample"
	"./service"
	"google.golang.org/grpc"
)

func main() {
    listenPort, err := net.Listen("tcp", ":19003")
    if err != nil {
        log.Fatalln(err)
    }
    server := grpc.NewServer()
    sampleService := &service.SampleService{}
    protobuf.RegisterSampleServer(server, sampleService)
    server.Serve(listenPort)
}
