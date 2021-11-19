package main

import (
	"fmt"
	"os"
)

func writeData() {
	err := os.WriteFile("file.txt", []byte("abcdefg\nhijklmn\nopqrstu\nvwxyz"), os.ModePerm)
	if err != nil {
		fmt.Println(err)
	}
}

func readData() {
	file, err := os.ReadFile("file.txt")

	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Println(string(file))
		// =>
		// abcdefg
		// hijklmn
		// opqrstu
		// vwxyz
	}
}

func main() {
	writeData()
	readData()
}
