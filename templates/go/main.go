package main

import (
	"fmt"
	"runtime"
)

func main() {
	fmt.Println("Hello from {{name}}!")
	fmt.Printf("Go %s on %s/%s, inside a Hyperlight micro-VM\n", runtime.Version(), runtime.GOOS, runtime.GOARCH)
}
