package main

import (
	"fmt"
	"os"
	"strings"
	"time"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
)

func setLocation(timezone string) error {
	location, err := time.LoadLocation(timezone)
	if err != nil {
		return err
	}
	time.Local = location
	return nil
}

func makeResponse(message string) events.APIGatewayProxyResponse {
	return events.APIGatewayProxyResponse{
		Body:       message,
		StatusCode: 200,
	}
}

// 集合を求める
func getFamilyOfSetsOfMapKey(a *map[string]string, b *map[string]string) ([]string, []string, []string) {
	var setDifferenceA []string
	var setDifferenceB []string
	var intersection []string
	for aKey := range *a {
		_, found := (*b)[aKey]
		if !found {
			setDifferenceA = append(setDifferenceA, aKey)
		} else {
			intersection = append(intersection, aKey)
		}
	}
	for bKey := range *b {
		if _, found := (*a)[bKey]; !found {
			setDifferenceB = append(setDifferenceB, bKey)
		}
	}
	return setDifferenceA, setDifferenceB, intersection
}

func handler(request events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {

	// ログが日本時間で出力されるように設定
	if err := setLocation(`Asia/Tokyo`); err != nil {
		panic(err)
	}

	// Sambaへの接続設定
	samba := Samba{
		ip:       os.Getenv(`SMB_IP`),
		port:     os.Getenv(`SMB_PORT`),
		share:    os.Getenv(`SMB_SHARE`),
		prefix:   os.Getenv(`SMB_PREFIX`),
		user:     os.Getenv(`SMB_USER`),
		password: os.Getenv(`SMB_PASS`),
	}

	// Sambaから監視対象のディレクトリ内のファイルのハッシュ値を取得する
	sambaChecksums, err := samba.Ls()
	if err != nil {
		panic(err)
	}

	// S3への接続設定
	s3 := S3{
		AwsKey:      os.Getenv(`AWS_KEY`),
		AwsSecret:   os.Getenv(`AWS_SEACRET`),
		AwsUrl:      os.Getenv(`AWS_URL`),
		S3Bucket:    os.Getenv(`S3_BUCKET_NAME`),
		S3KeyPrefix: os.Getenv(`S3_KEY_PREFIX`),
	}

	// debug <<<<<<<<
	// data := strings.NewReader("sample")
	// data2 := strings.NewReader("sample")
	// sum, err := Check(data2)
	// fmt.Println(sum)
	// if err = s3.PutObject(`hogehoge/sample.txt`, data); err != nil {
	// 	panic(err)
	// }
	// if err = s3.LsBuckets(); err != nil {
	// 	panic(err)
	// }
	// >>>>>>>> debug

	// S3から監視対象のディレクトリ内のファイルのハッシュ値を取得する
	s3Checksums, err := s3.LsKey()
	if err != nil {
		panic(err)
	}

	// SambaとS3それぞれのハッシュ値を比較する
	// FIXME: 並列でできそう
	a, b, i := getFamilyOfSetsOfMapKey(&sambaChecksums, &s3Checksums)

	for _, checkTarget := range i {
		// チェックサムが異なる場合は更新されているものとみなす
		if sambaChecksums[checkTarget] != s3Checksums[checkTarget] {
			a = append(a, checkTarget)
		}
	}

	fmt.Println(`put s3 target:    `, a)
	fmt.Println(`delete s3 target: `, b)
	fmt.Println(`not work target:  `, i)

	// put
	// 監視対象のファイルが存在しない場合はS3にファイルをアップロードする
	for _, putTarget := range a {
		fmt.Println(`putting:`, putTarget)
		read, err := samba.GetData(putTarget)
		if err != nil {
			panic(err)
		}
		if err = s3.PutObject(putTarget, strings.NewReader(string(read))); err != nil {
			panic(err)
		}
	}

	// delete
	// sambda側に存在していない場合はS3からファイルを削除する
	for _, deleteTarget := range b {
		s3.DeleteObject(deleteTarget)
	}

	// FIXME: この時、処理前と処理後のハッシュ値を取得して差があった場合はエラーにするとより綿密なシステムになるかも

	return makeResponse("worked."), nil
}

func main() {
	lambda.Start(handler)
}
