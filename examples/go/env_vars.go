package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Printf("MY_VAR=%s\n", os.Getenv("MY_VAR"))
	fmt.Printf("DEBUG=%s\n", os.Getenv("DEBUG"))
	fmt.Printf("GREETING=%s\n", os.Getenv("GREETING"))
}
