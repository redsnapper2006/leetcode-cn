package main

import (
	"fmt"
)

func wateringPlants(plants []int, capacity int) int {
	count := 0
	remain := capacity
	for i := 0; i < len(plants); i++ {
		if remain >= plants[i] {
			count++
			remain -= plants[i]
		} else {
			count += 2*i + 1
			remain = capacity - plants[i]
		}
	}
	return count
}

func main() {
	fmt.Println()
}
