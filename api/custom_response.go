package api

import (
	"encoding/json"
	"net/http"
)

type JiaozifsResponse struct {
	http.ResponseWriter
}

func (response *JiaozifsResponse) RespJSON(v interface{}) {
	data, err := json.Marshal(v)
	if err != nil {
		response.RespError(err)
		return
	}
	response.Header().Set("Content-Type", "application/json")
	response.WriteHeader(http.StatusOK)
	_, _ = response.Write(data)
}

func (response *JiaozifsResponse) RespError(err error) {
	response.WriteHeader(http.StatusOK)
	response.Write([]byte(err.Error()))
}
