package connect

import (
	"crud/sqlconv"
	"database/sql"
	"fmt"

	_ "github.com/lib/pq"
	_ "github.com/go-sql-driver/mysql"
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
	case "mysql":
		dataSource = fmt.Sprintf("%v:%v@(%v)/%v", crud.User, crud.Password, crud.Host, crud.Database)
	default:
		return nil, fmt.Errorf("unknown driver. ( %v )", crud.Driver)
	}

	return sql.Open(crud.Driver, dataSource)
}

func (crud *DatabaseInformation) Create(insertString string, objects interface{}) error {

	db, err := crud.openDatabase()
	if err != nil {
		return fmt.Errorf("crud: Failed to connect to the database. ( %v )", err)
	}

	transaction, err := db.Begin()
	if err != nil {
		return fmt.Errorf("crud: Failed to make transaction. ( %v )", err)
	}

	insertResult, err := transaction.Query(insertString)
	if err != nil {
		return fmt.Errorf("crud: Failed to set insert query. ( %v )", err)
	}
	defer insertResult.Close()

	var selectString string
	switch crud.Driver {
	case "postgres":
		selectString = fmt.Sprintf("SELECT * FROM %v WHERE id=LASTVAL();", crud.Table)
	case "mysql":
		selectString = fmt.Sprintf("SELECT * FROM %v WHERE id=LAST_INSERT_ID;", crud.Table)
	default:
		return fmt.Errorf("unknown driver. ( %v )", crud.Driver)
	}

	selectResult, err := transaction.Query(selectString)
	if err != nil {
		rollbackErr := transaction.Rollback()
		return fmt.Errorf("crud: Failed to set select query. ( %v ), ( Rollback result: %v )", err, rollbackErr)
	}
	defer selectResult.Close()

	err = sqlconv.Unmarshal(selectResult, objects)
	if err != nil {
		return fmt.Errorf("crud: Failed to unmarshal. ( %v )", err)
	}

	err = transaction.Commit()
	if err != nil {
		return fmt.Errorf("crud: Failed to commit transaction. ( %v )", err)
	}

	return nil
}

func (crud *DatabaseInformation) Read(selectString string, objects interface{}) error {

	db, err := crud.openDatabase()
	if err != nil {
		return fmt.Errorf("crud: Failed to connect to the database. ( %v )", err)
	}

	selectResult, err := db.Query(selectString)
	if err != nil {
		return fmt.Errorf("crud: Failed to request. ( %v )", err)
	}
	defer selectResult.Close()

	err = sqlconv.Unmarshal(selectResult, objects)
	if err != nil {
		return fmt.Errorf("crud: Failed to unmarshal. ( %v )", err)
	}

	return nil
}
