package handlers

import (
	"fmt"
	"net/http"

	"github.com/go-chi/chi"
	chimiddle "github.com/go-chi/chi/middleware"
	"github.com/go-chi/render"
	"github.com/promethean-tech/go-vs-node-api/tree/main/go/interal/middleware"
)

func Handler(r *chi.Mux) {
	// global stripSlash middleware
	r.Use(chimiddle.StripSlashes)

	// r.Route("/coins", func(router chi.Router) {
	// 	router.Use(middleware.Authorization)

	// 	router.Get("/{userID}", GetCoinBalance)

	// })

	r.Route("/coins/{userID}", func(router chi.Router) {
		router.Use(middleware.Authorization)
	


		
		router.Get("/", GetCoinBalance)
	})

	// test route
	r.Route("/test", func(router chi.Router) {
		
		router.Get("/", testHandler)
		

	})
}

// TestResponse is a sample response struct
type TestResponse struct {
	Message string `json:"message"`
}


func testHandler(w http.ResponseWriter, r *http.Request) {
	response := TestResponse{Message: "Hello, Chi API!"}

	// Set content type to JSON
	w.Header().Set("Content-Type", "application/json")

	fmt.Fprint(w, "TESTING FROM GO")

	// Use chi's render package to render and send JSON response
	render.JSON(w, r, response)
}