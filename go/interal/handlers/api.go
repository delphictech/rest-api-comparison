package handlers

import (
	"github.com/go-chi/chi"
	chimiddle "github.com/go-chi/chi/middleware"
	"github.com/promethean-tech/go-vs-node-api/tree/main/go/interal/middleware"
)

func Handler(r *chi.Mux) {
	// global stripSlash middleware
	r.Use(chimiddle.StripSlashes)

	r.Route("/account", func(router chi.Router) {
		router.Use(middleware.Authorization)

		router.GET("coins", GetCoinBalance)
	})
}