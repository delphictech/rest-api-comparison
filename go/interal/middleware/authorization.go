package middleware

import (
	"errors"
	"net/http"

	"github.com/go-chi/chi"
	"github.com/promethean-tech/go-vs-node-api/tree/main/go/interal/tools"
)

var ErrorUnAuthorized = errors.New("invalidusername or token")

// middleware function that checks user auth before proceeding
func Authorization(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {

		// get token and username from headers/params
		var username = chi.URLParam(r, "userID")

		var token = r.Header.Get("authtoken")

		userData := tools.MockLoginDetails

		if userData[username].AuthToken == token {
			next.ServeHTTP(w, r)
			return
		}

		// throw error if incorrect auth
		http.Error(w, "Forbidden", http.StatusForbidden)
		// log.Error(ErrorUnAuthorized)
		// api.RequestErrorHandler(w, ErrorUnAuthorized)

	})

}
