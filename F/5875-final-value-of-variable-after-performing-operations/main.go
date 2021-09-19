package main

import "fmt"

func finalValueAfterOperations(operations []string) int {
	x := 0

	for _, oper := range operations {
		for _, b := range oper {
			if byte(b) == byte('+') {
				x++
				break
			} else if byte(b) == byte('-') {
				x--
				break
			}
		}
	}

	return x
}

func main() {
	fmt.Println()
}
