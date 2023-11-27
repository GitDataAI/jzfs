package api_impl

import (
	"encoding/json"
	"net/http"
)

func writeJson(w http.ResponseWriter, v interface{}) {
	data, err := json.Marshal(v)
	if err != nil {
		writeError(w, err)
		return
	}
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	_, _ = w.Write(data)
}

func writeError(w http.ResponseWriter, err error) {
	w.WriteHeader(http.StatusOK)
	w.Write([]byte(err.Error()))
}
