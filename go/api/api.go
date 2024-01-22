package api

import (
	"encoding/json"
	"net/http"
)

// referncing the golang-standards/api/README.md
// We can see that the api folder is similiar to a utils or types folder in ts
// This folder should define certain parts of our api. In our case just statically typing our api

type CoinBalanceParams struct {
	Username string
}

type CoinBalanceResponse struct {
	Code    int
	Balance int64
}

type Error struct {
	Code    int
	Message string
}

// function that will set the headers and send a response to the client if there is an error
func writeError(w http.ResponseWriter, code int, message string) {
	resp := Error{
		Code:    code,
		Message: message,
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(code)

	json.NewEncoder(w).Encode(resp)

}

// export the error functions 
var (
	RequestErrorHandler = func(w http.ResponseWriter, err error) {
		writeError(w, http.StatusBadRequest, err.Error())
	}

	InternalErrorHandler = func(w http.ResponseWriter) {
		writeError(w, http.StatusInternalServerError, "An unexpected Error occured")
	}
)
