package main

import (
	"fmt"
	"net/http"

	"github.com/go-chi/chi"
	"github.com/promethean-tech/go-vs-node-api/tree/main/go/interal/handlers"
	log "github.com/sirupsen/logrus"
)

func main() {

	r := chi.NewRouter()
	// var r *chi.Mux = chi.NewRouter()

	// r := &re

	handlers.Handler(r)

	// Define a handler function for the "/" route
	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		// Send the response "HELLO FROM GO"
		fmt.Fprint(w, "HELLO FROM GO")
	})

	

	// Start the HTTP server on localhost:8000
	port := 8000
	fmt.Printf("Server is running on http://localhost:%d\n", port)
	err := http.ListenAndServe(fmt.Sprintf(":%d", port), r)

if err != nil {
	log.Error((err))
}
}