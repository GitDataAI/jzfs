package api

import (
	"encoding/json"
	"net/http"
)

type JiaozifsResponse struct {
	http.ResponseWriter
}

func (response *JiaozifsResponse) RespJSON(v interface{}) {
	response.Header().Set("Content-Type", "application/json")
	response.WriteHeader(http.StatusOK)
	err := json.NewEncoder(response).Encode(v)
	if err != nil {
		response.RespError(err)
		return
	}
}

func (response *JiaozifsResponse) RespError(err error) {
	response.WriteHeader(http.StatusOK)
	_, _ = response.Write([]byte(err.Error()))
}
