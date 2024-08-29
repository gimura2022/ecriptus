package main

import "C"

//export x2
func x2(arg C.int) C.int {
    return arg * 2
}

func main() {} // This should be present to satisfy the Go build
