package main

import (
	"fmt"
	"io"
	"net/http"
)

func main() {
	request, err := http.NewRequest(http.MethodGet, "https://example.com/", nil)
	if err != nil {
		fmt.Println(err)
		return
	}
	response, err := http.DefaultClient.Do(request)

	if err != nil {
		fmt.Println(err)
		return
	}

	// HTTP Status Code
	fmt.Println(response.StatusCode)

	// Body
	body, _ := io.ReadAll(response.Body)
	fmt.Println(string(body))
}
