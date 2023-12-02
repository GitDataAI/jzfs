package api

import (
	"encoding/json"
	"net/http"
)

type JiaozifsResponse struct {
	http.ResponseWriter
}

func (response *JiaozifsResponse) JSON(v interface{}) {
	response.Header().Set("Content-Type", "application/json")
	response.WriteHeader(http.StatusOK)
	err := json.NewEncoder(response).Encode(v)
	if err != nil {
		response.Error(err)
		return
	}
}

func (response *JiaozifsResponse) Error(err error) {
	response.WriteHeader(http.StatusInternalServerError)
	_, _ = response.Write([]byte(err.Error()))
}

func (response *JiaozifsResponse) CodeMsg(code int, msg string) {
	response.WriteHeader(code)
	_, _ = response.Write([]byte(msg))
}
