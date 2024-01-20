package handlers

import (
	"encoding/json"
	"net/http"

	"github.com/go-chi/chi"
	"github.com/promethean-tech/go-vs-node-api/tree/main/go/api"
	log "github.com/sirupsen/logrus"
)

func GetCoinBalance(w http.ResponseWriter, r *http.Request) {

	var username string = chi.URLParam(r, "userID")

	var token = r.Header.Get("authtoken")

	
	response := map[string]string {
		"userName": username,
		"token": token,
	}

	err := json.NewEncoder(w).Encode(response)

	 if err != nil {
		log.Error(err)
		api.InternalErrorHandler(w)
		return
	 }
}