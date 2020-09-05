package main

import "fmt"

func restoreString(s string, indices []int) string {
	buf := make([]byte, len(s))
	for i, b := range s {
		buf[indices[i]] = byte(b)
	}
	return string(buf)
}

func main() {
	fmt.Println("a")
}
