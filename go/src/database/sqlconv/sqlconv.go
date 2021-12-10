package sqlconv

import (
	"database/sql"
	"fmt"
	"reflect"
)

func rowsToMap(rows *sql.Rows) ([]map[string]interface{}, error) {
	var results []map[string]interface{}
	columns, err := rows.Columns()
	model := make([]interface{}, len(columns))

	for rows.Next() {
		var entity = append(model)
		var pointers = append(model)
		for index, _ := range columns {
			pointers[index] = &entity[index]
		}

		rows.Scan(pointers...)
		result := make(map[string]interface{})
		for index, column := range columns {
			result[column] = entity[index]
		}

		results = append(results, result)
	}
	return results, err
}

func mapToStruct(structObjects interface{}, mapObjects []map[string]interface{}) error {
	structTagType := "sql"

	reflectPointerValue := reflect.ValueOf(structObjects)
	if reflectPointerValue.Kind() != reflect.Ptr || reflectPointerValue.IsNil() {
		return fmt.Errorf("Interface is not Pointer. ( %v )", reflectPointerValue.Kind())
	}

	reflectValue := reflect.Indirect(reflectPointerValue)
	if reflectValue.Type().Kind() != reflect.Slice {
		return fmt.Errorf("Interface is not Slice Pointer. ( %v )", reflectValue.Type().Kind())
	}

	structEntity := reflect.Indirect(reflect.New(reflectValue.Type().Elem()))
	if structEntity.Type().Kind() != reflect.Struct {
		return fmt.Errorf("Slice's Element is not Struct. ( %v )", structEntity.Type().Kind())
	}

	for _, mapObject := range mapObjects {
		for index := 0; index < structEntity.Type().NumField(); index++ {
			fieldType := structEntity.Type().Field(index)
			mapValue := mapObject[fieldType.Tag.Get(structTagType)]
			if mapValue == nil {
				return fmt.Errorf("Tag is not found. ( %v )", structTagType)
			}

			fieldValue := structEntity.FieldByName(fieldType.Name)

			switch fieldValue.Kind() {
			case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64, reflect.Uint, reflect.Uint8, reflect.Uint16, reflect.Uint32, reflect.Uint64:
				fieldValue.SetInt(mapValue.(int64))
			case reflect.Bool:
				fieldValue.SetBool(mapValue.(bool))
			case reflect.Float32, reflect.Float64:
				fieldValue.SetFloat(mapValue.(float64))
			case reflect.String:
				fieldValue.SetString(mapValue.(string))
			case reflect.Struct:
				fieldValue.Set(reflect.ValueOf(mapValue))
			default:
				return fmt.Errorf("Struct's member is an invalid type. ( MemberType:%v, MemberTagName:%v, SetValue:%v )", fieldValue.Kind(), fieldType.Tag.Get("sql"), mapValue)
			}
		}
		sliceEntity := reflectPointerValue.Elem()
		sliceEntity.Set(reflect.Append(sliceEntity, structEntity))
	}

	return nil
}

func Unmarshal(rows *sql.Rows, structObjects interface{}) error {
	mapObjects, err := rowsToMap(rows)
	if err != nil {
		return err
	}

	err = mapToStruct(structObjects, mapObjects)
	if err != nil {
		return err
	}
	return nil
}
