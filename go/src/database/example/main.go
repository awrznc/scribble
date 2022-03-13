package main

import (
	"crud/connect"
	"fmt"
	"time"
)

type ExampleTable struct {
	Id       int       `sql:"id"`
	Label    string    `sql:"label"`
	X        float64   `sql:"x"`
	Y        float64   `sql:"y"`
	CreateAt time.Time `sql:"create_at"`
	UpdateAt time.Time `sql:"update_at"`
}

func main() {
	crudObject := connect.DatabaseInformation{"example_user", "example_password", "localhost", "postgres", "example_db", "example_table"}

	var exampleTable []ExampleTable
	query := fmt.Sprintf("SELECT * FROM %v", crudObject.Table)
	err := crudObject.Read(query, &exampleTable)
	if err != nil {
		fmt.Println(err)
	}

	// Print result.
	for _, value := range exampleTable {
		fmt.Println("==== ==== ==== ====")

		fmt.Println("Id       :", value.Id)
		fmt.Println("Label    :", value.Label)
		fmt.Println("X        :", value.X)
		fmt.Println("Y        :", value.Y)
		fmt.Println("CreateAt :", value.CreateAt)
		fmt.Println("UpdateAt :", value.UpdateAt)
	}
	// =>
	// ==== ==== ==== ====
	// Id       : 1
	// Label    : sample point
	// X        : 10
	// Y        : 12
	// CreateAt : 2021-12-09 12:04:05.350302 +0000 +0000
	// UpdateAt : 2021-12-09 12:04:05.350302 +0000 +0000
	// ==== ==== ==== ====
	// Id       : 2
	// Label    : sample point2
	// X        : 109.1
	// Y        : 120.2
	// CreateAt : 2021-12-10 09:27:42.005203 +0000 +0000
	// UpdateAt : 2021-12-10 09:27:42.005203 +0000 +0000

	query = fmt.Sprintf("INSERT INTO %v(label, x, y) VALUES('sample point', 1.0, 12.2);", crudObject.Table)
	err = crudObject.Create(query, &exampleTable)
	if err != nil {
		fmt.Println(err)
	}
}
