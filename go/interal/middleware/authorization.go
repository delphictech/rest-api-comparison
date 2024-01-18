package middleware

import (
	"errors"
	"net/http"

	"github.com/promethean-tech/go-vs-node-api/tree/main/go/api"
	"github.com/promethean-tech/go-vs-node-api/tree/main/go/interal/tools"
	log "github.com/sirupsen/logrus"
)

var ErrorUnAuthorized = errors.New("invalidusername or token")

func Authorization(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {

		var username string = r.URL.Query().Get("username")

		var token = r.Header.Get("authtoken")

	

		if username == "" || token == ""  {
			log.Error(ErrorUnAuthorized)
			api.RequestErrorHandler(w, ErrorUnAuthorized)
		}

		userData := tools.MockLoginDetails

		log.Debug(userData)

		next.ServeHTTP(w, r)

	})



}