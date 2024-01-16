package middleware

import (
	"errors"
	"net/http"
)

var ErrorUnAuthorized = errors.New("invalidusername or token")

func Authorization(next http.Handler) {
	return http.HandleFunc(func(w http.ResponseWriter, r *http.Request) {

		// var username string = r.URL.Query().Get("username")

		// var token = r.Header.Get("authtoken")

		// var err error

		// if username == "" || token == ""  {


		// }

		

		

	})



}