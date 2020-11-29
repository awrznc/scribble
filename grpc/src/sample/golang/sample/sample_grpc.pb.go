// Code generated by protoc-gen-go-grpc. DO NOT EDIT.

package sample

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion7

// SampleClient is the client API for Sample service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type SampleClient interface {
	GetSample(ctx context.Context, in *Hoge, opts ...grpc.CallOption) (*Piyo, error)
}

type sampleClient struct {
	cc grpc.ClientConnInterface
}

func NewSampleClient(cc grpc.ClientConnInterface) SampleClient {
	return &sampleClient{cc}
}

func (c *sampleClient) GetSample(ctx context.Context, in *Hoge, opts ...grpc.CallOption) (*Piyo, error) {
	out := new(Piyo)
	err := c.cc.Invoke(ctx, "/sample.Sample/GetSample", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// SampleServer is the server API for Sample service.
// All implementations must embed UnimplementedSampleServer
// for forward compatibility
type SampleServer interface {
	GetSample(context.Context, *Hoge) (*Piyo, error)
	mustEmbedUnimplementedSampleServer()
}

// UnimplementedSampleServer must be embedded to have forward compatible implementations.
type UnimplementedSampleServer struct {
}

func (UnimplementedSampleServer) GetSample(context.Context, *Hoge) (*Piyo, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetSample not implemented")
}
func (UnimplementedSampleServer) mustEmbedUnimplementedSampleServer() {}

// UnsafeSampleServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to SampleServer will
// result in compilation errors.
type UnsafeSampleServer interface {
	mustEmbedUnimplementedSampleServer()
}

func RegisterSampleServer(s grpc.ServiceRegistrar, srv SampleServer) {
	s.RegisterService(&_Sample_serviceDesc, srv)
}

func _Sample_GetSample_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(Hoge)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(SampleServer).GetSample(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/sample.Sample/GetSample",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(SampleServer).GetSample(ctx, req.(*Hoge))
	}
	return interceptor(ctx, in, info, handler)
}

var _Sample_serviceDesc = grpc.ServiceDesc{
	ServiceName: "sample.Sample",
	HandlerType: (*SampleServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "GetSample",
			Handler:    _Sample_GetSample_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "sample.proto",
}