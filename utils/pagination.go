package utils

import (
	"reflect"
	"strconv"
	"time"
)

const DefaultMaxPerPage int = 1000

type PageManage struct {
	HasMore    bool
	MaxPerPage int
	NextOffset string
	Results    int
}

func PaginationFor(hasMore bool, results interface{}, fieldName string) PageManage {
	pagination := PageManage{
		HasMore:    hasMore,
		MaxPerPage: DefaultMaxPerPage,
	}

	s := reflect.ValueOf(results)
	pagination.Results = s.Len()
	if !hasMore || pagination.Results == 0 {
		return pagination
	}
	v := s.Index(pagination.Results - 1)
	token := v.FieldByName(fieldName)
	switch token.Kind() {
	case reflect.Struct:
		if token.Type() == reflect.TypeOf(time.Time{}) {
			pagination.NextOffset = token.Interface().(time.Time).String()
		}
	case reflect.Int64:
		pagination.NextOffset = strconv.FormatInt(token.Int(), 10)
	case reflect.String:
		pagination.NextOffset = token.String()
	}
	return pagination
}
