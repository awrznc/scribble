package main

import (
	"html/template"
	"strings"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
)

type Template struct {
	Title string
	Body  string
}

func handler(request events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {
	parsedObject, err := template.ParseFiles("/opt/index.html")
	if err != nil {
		panic(err)
	}

	template := Template{"title_string", "body_string"}
	writer := new(strings.Builder)
	err = parsedObject.Execute(writer, template)
	if err != nil {
		panic(err)
	}

	return events.APIGatewayProxyResponse{
		Body: writer.String(),
		Headers: map[string]string{
			"Content-Type": "text/html; charset=utf-8",
		},
		StatusCode: 200,
	}, nil
}

func main() {
	lambda.Start(handler)
}
