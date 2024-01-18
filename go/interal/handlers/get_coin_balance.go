package handlers

import (
	"encoding/json"
	"net/http"

	"github.com/promethean-tech/go-vs-node-api/tree/main/go/api"
	log "github.com/sirupsen/logrus"
)

func GetCoinBalance(w http.ResponseWriter, r *http.Request) {
	var response = api.CoinBalanceResponse {
		Balance: 123,
		Code: 200,
	}
	err := json.NewEncoder(w).Encode(response)

	 if err != nil {
		log.Error(err)
		api.InternalERrorHandler(w)
		return
	 }
}