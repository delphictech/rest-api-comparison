package handlers

import (
	"net/http"

	"github.com/go-chi/render"
)

// TestResponse is a sample response struct
type TestResponse struct {
	Message string `json:"message"`
	Code    int64  `json:"code"`
}

func TestHandler(w http.ResponseWriter, r *http.Request) {
	response := TestResponse{Message: "testing route", Code: 200}

	// Set content type to JSON
	w.Header().Set("Content-Type", "application/json")

	// Use chi's render package to render and send JSON response
	render.JSON(w, r, response)
}
