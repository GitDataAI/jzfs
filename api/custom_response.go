package api

import (
	"encoding/json"
	"net/http"
)

type JiaozifsResponse struct {
	http.ResponseWriter
}

// JSON convert object to json format and write to response,
// if not specific code, default code is 200. given code will
// overwrite default code, if more than one code, the first one will be used.
func (response *JiaozifsResponse) JSON(v any, code ...int) {
	if len(code) == 0 {
		response.WriteHeader(http.StatusOK)
	} else {
		response.WriteHeader(code[0])
	}
	response.Header().Set("Content-Type", "application/json")
	err := json.NewEncoder(response.ResponseWriter).Encode(v)
	if err != nil {
		response.Error(err)
		return
	}
}

// OK response with 200
func (response *JiaozifsResponse) OK() {
	response.WriteHeader(http.StatusOK)
}

// NotFound response with 404
func (response *JiaozifsResponse) NotFound() {
	response.WriteHeader(http.StatusNotFound)
}

// Error response with 500 and error message
func (response *JiaozifsResponse) Error(err error) {
	response.WriteHeader(http.StatusInternalServerError)
	_, _ = response.Write([]byte(err.Error()))
}

// String response and string
// if not specific code, default code is 200. given code will
// overwrite default code, if more than one code, the first one will be used.
func (response *JiaozifsResponse) String(msg string, code ...int) {
	if len(code) == 0 {
		response.WriteHeader(http.StatusOK)
	} else {
		response.WriteHeader(code[0])
	}
	response.Header().Set("Content-Type", "text/plain;charset=UTF-8")
	_, _ = response.Write([]byte(msg))
}

// Code response with uncommon code
func (response *JiaozifsResponse) Code(code int) {
	response.WriteHeader(code)
}
