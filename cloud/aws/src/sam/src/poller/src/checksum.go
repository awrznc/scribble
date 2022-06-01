package main

import (
	"crypto/md5"
	"crypto/sha256"
	"fmt"
	"io"
	"os"
)

func GetSha256FileChecksum(targetFilePath string) (string, error) {
	file, err := os.Open(targetFilePath)
	if err != nil {
		return "", err
	}
	defer file.Close()

	hash := sha256.New()
	if _, err := io.Copy(hash, file); err != nil {
		return "", err
	}

	return fmt.Sprintf("%x", hash.Sum(nil)), nil
}

func Check(data io.Reader) (string, error) {
	// hash := sha256.New()
	hash := md5.New()
	if _, err := io.Copy(hash, data); err != nil {
		return "", err
	}

	return fmt.Sprintf("%x", hash.Sum(nil)), nil
}
