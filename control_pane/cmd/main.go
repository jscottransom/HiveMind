package main

import (
	"fmt"
	"net/http"
	"io"
)

func main() {
	http.HandleFunc("/", handleRoot)
	fmt.Println("HiveMind Server running on port 8080")
	err := http.ListenAndServe(":8080", nil)
	if err != nil {
		startupError := fmt.Errorf("Error starting up on Port 8080")
		fmt.Println(startupError)
	}
}

func handleRoot(w http.ResponseWriter, r *http.Request) {
	io.WriteString(w, "Welcome to HiveMind\n")
}