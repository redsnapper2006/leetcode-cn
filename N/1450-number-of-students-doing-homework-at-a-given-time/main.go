package main

import "fmt"

func busyStudent(startTime []int, endTime []int, queryTime int) int {
	c := 0

	for i := 0; i < len(startTime); i++ {
		if startTime[i] <= queryTime && queryTime <= endTime[i] {
			c++
		}
	}
	return c
}

func main() {
	fmt.Println("a")
}
