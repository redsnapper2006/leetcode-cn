package main

import (
	"fmt"
	"sort"
)

func minMovesToSeat(seats []int, students []int) int {
	sort.Ints(seats)
	sort.Ints(students)
	sum := 0
	for i := 0; i < len(seats); i++ {
		d := students[i] - seats[i]
		if d < 0 {
			d = -d
		}
		sum += d
	}
	return sum
}

func main() {
	fmt.Println()
}
