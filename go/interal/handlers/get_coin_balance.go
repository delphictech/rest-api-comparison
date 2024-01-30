package handlers

import (
	"encoding/json"
	"net/http"

	"github.com/promethean-tech/go-vs-node-api/tree/main/go/interal/tools"

	"github.com/go-chi/chi"
	"github.com/promethean-tech/go-vs-node-api/tree/main/go/api"
	log "github.com/sirupsen/logrus"
)

func GetCoinBalance(w http.ResponseWriter, r *http.Request) {

	var username string = chi.URLParam(r, "userID")

	userData := tools.MockCoinDetails[username].Balance

	response := map[string]api.CoinBalanceResponse{"data": {
		UserName: username,
		Balance:  userData},
	}

	err := json.NewEncoder(w).Encode(response)

	if err != nil {
		log.Error(err)
		api.InternalErrorHandler(w)
		return
	}
}
