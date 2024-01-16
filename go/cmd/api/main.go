package main

import( 
	"fmt"
	"net/http"

	"github.com/go-chi/chi"
	log "github.com/sirupsen/logrus"
)

func main() {

	router := chi.NewRouter()

	// Define a handler function for the "/" route
	router.Get("/", func(w http.ResponseWriter, r *http.Request) {
		// Send the response "HELLO FROM GO"
		fmt.Fprint(w, "HELLO FROM GO")
	})

	// Start the HTTP server on localhost:8000
	port := 8000
	fmt.Printf("Server is running on http://localhost:%d\n", port)
	err := http.ListenAndServe(fmt.Sprintf(":%d", port), router)

if err != nil {
	log.Error((err))
}
}