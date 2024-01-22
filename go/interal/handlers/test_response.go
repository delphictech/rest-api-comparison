package handlers

import (
	"net/http"
	"fmt"
	"github.com/go-chi/render"
)

// TestResponse is a sample response struct
type TestResponse struct {
	Message string `json:"message"`
}

func TestHandler(w http.ResponseWriter, r *http.Request) {
	response := TestResponse{Message: "Hello, Chi API!"}

	// Set content type to JSON
	w.Header().Set("Content-Type", "application/json")

	fmt.Fprint(w, "TESTING FROM GO")

	// Use chi's render package to render and send JSON response
	render.JSON(w, r, response)
}