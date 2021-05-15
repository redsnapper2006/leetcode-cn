package main

import "fmt"

func memLeak(memory1 int, memory2 int) []int {
	seconds := 1

	for {
		if memory1 >= memory2 && memory1 >= seconds {
			memory1 -= seconds
		} else if memory2 >= seconds {
			memory2 -= seconds
		} else {
			break
		}
		seconds++
	}
	return []int{seconds, memory1, memory2}
}

func main() {
	fmt.Println()
}
