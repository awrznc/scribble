package connect

import (
	"crud/sqlconv"
	"database/sql"
	"fmt"
	_ "github.com/lib/pq"
)

type DatabaseInformation struct {
	User     string
	Password string
	Host     string
	Driver   string
	Database string
	Table    string
}

func (crud *DatabaseInformation) openDatabase() (*sql.DB, error) {
	var dataSource string

	switch crud.Driver {
	case "postgres":
		dataSource = fmt.Sprintf("user=%v password=%v host=%v dbname=%v", crud.User, crud.Password, crud.Host, crud.Database)
	default:
		dataSource = fmt.Sprintf("%v:%v@(%v)/%v", crud.User, crud.Password, crud.Host, crud.Database)
	}

	return sql.Open(crud.Driver, dataSource)
}

func (crud *DatabaseInformation) Read(objects interface{}) error {

	db, err := crud.openDatabase()
	if err != nil {
		return fmt.Errorf("crud: Failed to connect to the database. ( %v )", err)
	}

	query := fmt.Sprintf("SELECT * FROM %v", crud.Table)
	rows, err := db.Query(query)
	if err != nil {
		return fmt.Errorf("crud: Failed to request. ( %v )", err)
	}
	defer rows.Close()

	err = sqlconv.Unmarshal(rows, objects)
	if err != nil {
		return fmt.Errorf("crud: Failed to unmarshal. ( %v )", err)
	}

	return nil
}
