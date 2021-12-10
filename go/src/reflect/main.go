package main

import (
	"fmt"
	"reflect"
)

func Assign(object interface{}) error {
	reflectPointerValue := reflect.ValueOf(object)
	fmt.Println("reflectPointerValue:", reflectPointerValue)
	// => &{12345 aaaaa}

	if reflectPointerValue.Kind() != reflect.Ptr || reflectPointerValue.IsNil() {
		errorMessage := "Argument is not a pointer"
		return fmt.Errorf("%v: %v", errorMessage, reflectPointerValue.Kind())
	}

	reflectValue := reflect.Indirect(reflectPointerValue)
	fmt.Println("reflectValue:", reflectValue)
	// => {12345 aaaaa}

	reflectValueType := reflectValue.Type()
	fmt.Println("reflectValueType:", reflectValueType)
	// => main.SampleType

	if reflectValueType.Kind() != reflect.Struct {
		errorMessage := "Interface is not Struct."
		return fmt.Errorf("%v: %v", errorMessage, reflectValueType.Kind())
	}

	for index := 0; index < reflectValueType.NumField(); index++ {
		fmt.Println("======================")

		fieldType := reflectValueType.Field(index)
		fmt.Println("fieldType:", fieldType)
		// => {Id  int data:"id" 0 [0] false}

		fmt.Println("fieldType.Name:", fieldType.Name)
		// => Id

		fmt.Println("fieldType.Get(\"data\"):", fieldType.Tag.Get("data"))
		// => id

		fieldValue := reflectValue.FieldByName(fieldType.Name)
		fmt.Println("fieldValue:", fieldValue)
		// => 12345

		fmt.Println("fieldValue.Kind():", fieldValue.Kind())
		// => int

		switch fieldValue.Kind() {
		case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64:
			fieldValue.SetInt(987)
		case reflect.String:
			fieldValue.SetString("sample")
		}
	}
	return nil
}

type SampleType struct {
	Id   int    `data:"id"`
	Name string `data:"name"`
}

func main() {
	object := SampleType{12345, "aaaaa"}

	err := Assign(&object)
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println(object)
	// => {987 sample}
}
