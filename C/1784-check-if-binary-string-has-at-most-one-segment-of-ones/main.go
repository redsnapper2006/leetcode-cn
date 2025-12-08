package main

import "fmt"

func checkOnesSegment(s string) bool {
	isFirst := true
	for _, b := range s {
		if b == '0' {
			isFirst = false
		} else {
			if !isFirst {
				return false
			}
		}
	}
	return true
}

func main() {
	fmt.Println("")
}
