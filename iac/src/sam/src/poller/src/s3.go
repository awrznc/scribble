package main

import (
	"context"
	"fmt"
	"io"

	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/credentials"
	"github.com/aws/aws-sdk-go-v2/service/s3"
	"github.com/aws/aws-sdk-go-v2/service/s3/types"
)

type S3 struct {
	AwsKey      string
	AwsSecret   string
	AwsUrl      string
	S3Bucket    string
	S3KeyPrefix string
}

func (s3Object *S3) getConfig() (*s3.Client, error) {
	var optionFunctions []func(*config.LoadOptions) error

	if len(s3Object.AwsUrl) != 0 {
		resolver := aws.EndpointResolverWithOptionsFunc(func(service, region string, options ...interface{}) (aws.Endpoint, error) {
			return aws.Endpoint{
				URL:               s3Object.AwsUrl,
				HostnameImmutable: true,
			}, nil
		})
		optionFunctions = append(optionFunctions, config.WithEndpointResolverWithOptions(resolver))
	}

	if len(s3Object.AwsKey) != 0 || len(s3Object.AwsSecret) != 0 {
		provider := credentials.NewStaticCredentialsProvider(s3Object.AwsKey, s3Object.AwsSecret, ``)
		optionFunctions = append(optionFunctions, config.WithCredentialsProvider(provider))
	}

	config, err := config.LoadDefaultConfig(
		context.TODO(),
		optionFunctions...,
	)
	if err != nil {
		return nil, err
	}

	return s3.NewFromConfig(config), nil
}

func (s3Object *S3) LsBuckets() error {

	client, err := s3Object.getConfig()
	if err != nil {
		return err
	}

	result, err := client.ListBuckets(
		context.TODO(),
		&s3.ListBucketsInput{},
	)
	if err != nil {
		return err
	}

	for _, bucket := range result.Buckets {
		fmt.Println(*bucket.Name + `: ` + bucket.CreationDate.Format(`2006-01-02 15:04:05 Monday`))
	}
	return nil
}

func (s3Object *S3) LsKey() (map[string]string, error) {

	client, err := s3Object.getConfig()
	if err != nil {
		return nil, err
	}

	result, err := client.ListObjectsV2(context.TODO(), &s3.ListObjectsV2Input{
		Bucket: &s3Object.S3Bucket,
		Prefix: &s3Object.S3KeyPrefix,
	})
	if err != nil {
		return nil, err
	}

	hashmap := map[string]string{}

	for _, content := range result.Contents {
		checksum := *content.ETag
		hashmap[(*content.Key)[len(s3Object.S3KeyPrefix)+1:]] = checksum[1 : len(checksum)-1]
	}

	return hashmap, nil
}

func (s3Object *S3) PutObject(sufix string, data io.Reader) error {

	client, err := s3Object.getConfig()
	if err != nil {
		return err
	}

	key := s3Object.S3KeyPrefix + `/` + sufix
	_, err = client.PutObject(context.TODO(), &s3.PutObjectInput{
		Body:              data,
		Bucket:            &s3Object.S3Bucket,
		Key:               &key,
		ChecksumAlgorithm: types.ChecksumAlgorithmSha256,
	})

	if err != nil {
		return err
	}

	return nil
}

func (s3Object *S3) DeleteObject(sufix string) error {

	client, err := s3Object.getConfig()
	if err != nil {
		return err
	}

	key := s3Object.S3KeyPrefix + `/` + sufix
	_, err = client.DeleteObject(context.TODO(), &s3.DeleteObjectInput{
		Bucket: &s3Object.S3Bucket,
		Key:    &key,
	})

	if err != nil {
		return err
	}

	return nil
}
