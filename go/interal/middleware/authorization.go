package middleware

import (
	"errors"
	"fmt"
	"net/http"

	"github.com/go-chi/chi"
	"github.com/promethean-tech/go-vs-node-api/tree/main/go/api"
	"github.com/promethean-tech/go-vs-node-api/tree/main/go/interal/tools"
	log "github.com/sirupsen/logrus"
)

var ErrorUnAuthorized = errors.New("invalidusername or token")

func Authorization(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {

		var username = chi.URLParam(r, "userID")

		var token = r.Header.Get("authtoken")

		if username == "" {
			fmt.Printf("nothing")
		}

		userData := tools.MockLoginDetails

		if userData[username].AuthToken == token {
			next.ServeHTTP(w, r)
			return
		}

		log.Error(ErrorUnAuthorized)
		api.RequestErrorHandler(w, ErrorUnAuthorized)

	})

}
