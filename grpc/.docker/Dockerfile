FROM centos:8

RUN dnf install -y golang git unzip

WORKDIR /tmp
RUN curl -OL "https://github.com/protocolbuffers/protobuf/releases/download/v3.13.0/protoc-3.13.0-linux-x86_64.zip" && \
    unzip -o protoc-3.13.0-linux-x86_64.zip -d /usr/local

RUN go get -u google.golang.org/grpc && \
    go get -u github.com/golang/protobuf/protoc-gen-go && \
    go get -u github.com/pseudomuto/protoc-gen-doc/cmd/protoc-gen-doc
ENV PATH "$PATH:/root/go/bin/"

WORKDIR /home/src
