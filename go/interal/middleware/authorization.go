package middleware

import (
	// "strings"
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

		// Manually extract the last segment of the URL path as "userID"
		// segments := strings.Split(r.URL.Path, "/")
		// var username string
		// if len(segments) > 0 {
		// 	username = segments[len(segments)-1]
		// }

		var username = chi.URLParam(r, "userID")

		var token = r.Header.Get("authtoken")


		if username == "" {
			fmt.Printf("nothing")
		}


		if username == "" || token == ""  {
			log.Error(ErrorUnAuthorized)
			api.RequestErrorHandler(w, ErrorUnAuthorized)
		}

		userData := tools.MockLoginDetails

		fmt.Print(userData[username].AuthToken)


		if userData[username].AuthToken == token {
			next.ServeHTTP(w, r)

		}
		
		log.Error(ErrorUnAuthorized)
		api.RequestErrorHandler(w, ErrorUnAuthorized)


	})



}