package service

import (
    "context"
    "errors"
    protobuf "./../sample"
)

type SampleService struct {
}

func (s *SampleService) GetSample(ctx context.Context, message *protobuf.Hoge) (*protobuf.Piyo, error) {
    switch message.Flag {
    case "fugafuga":
        return &protobuf.Piyo{
            Name: "fugafuga",
            Kind: "ffuuggaa",
        }, nil
    case "sample":
        return &protobuf.Piyo{
            Name: "sample",
            Kind: "ssaammppllee",
        }, nil
    }
	return nil, errors.New("Not Found YourSample")
}
