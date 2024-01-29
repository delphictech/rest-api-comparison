package handlers

import (
	"github.com/go-chi/chi"
	chimiddle "github.com/go-chi/chi/middleware"

	"github.com/promethean-tech/go-vs-node-api/tree/main/go/interal/middleware"
)

// handler function used inside the main go function
func Handler(r *chi.Mux) {
	// global stripSlash middleware
	r.Use(chimiddle.StripSlashes)

	// test route
	r.Route("/test", func(router chi.Router) {

		router.Get("/", TestHandler)

	})

	// actual use case where middleware has to check auth before allowing route
	r.Route("/coins/{userID}", func(router chi.Router) {
		// the .Use is how you define middleware in go for specific route.
		router.Use(middleware.Authorization)

		// Will get the coinbalance if the middleware passes
		router.Get("/", GetCoinBalance)
	})
}
