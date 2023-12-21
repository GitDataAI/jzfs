package utils

import (
	"reflect"
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
	switch fieldName {
	case "UpdatedAt":
		pagination.NextOffset = token.Interface().(time.Time).String()
	case "Name":
		pagination.NextOffset = token.Interface().(string)
	}
	return pagination
}
