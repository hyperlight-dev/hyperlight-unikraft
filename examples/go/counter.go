package main

// A small HTTP server that counts the requests it has answered.  It is the
// guest entry point in tests/step.rs: a plain program with no driver, kept
// alive by the host's step pump, whose in-memory count must survive a
// checkpoint and restore.

import (
	"flag"
	"fmt"
	"net/http"
)

func main() {
	port := flag.Int("port", 8080, "TCP port to listen on")
	flag.Parse()

	count := 0
	http.HandleFunc("/readyz", func(w http.ResponseWriter, _ *http.Request) {
		fmt.Fprint(w, "ok")
	})
	http.HandleFunc("/", func(w http.ResponseWriter, _ *http.Request) {
		count++
		fmt.Fprintf(w, "count: %d\n", count)
	})
	if err := http.ListenAndServe(fmt.Sprintf(":%d", *port), nil); err != nil {
		panic(err)
	}
}
