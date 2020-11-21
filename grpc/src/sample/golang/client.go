package main
import (
    "context"
    "fmt"
    "log"
    protobuf "./sample"
    "google.golang.org/grpc"
)

func main() {
    conn, err := grpc.Dial("cxx:50051", grpc.WithInsecure())
    if err != nil {
        log.Fatal("client connection error:", err)
    }
    defer conn.Close()
    client := protobuf.NewSampleClient(conn)
    message := &protobuf.Hoge{ Flag: "sample" }
    response, err := client.GetSample(context.TODO(), message)

    // print error
    fmt.Printf("error::%#v \n", err)

    // print result
    fmt.Printf("result.Name:%#v \n", response.Name)
    fmt.Printf("result.Kind:%#v \n", response.Kind)
}
