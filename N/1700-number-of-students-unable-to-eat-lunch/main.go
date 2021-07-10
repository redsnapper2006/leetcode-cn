package main

import "fmt"

func countStudents(students []int, sandwiches []int) int {
	swIdx, sIdx := 0, 0

	for swIdx < len(sandwiches) {
		isFound := false
		idx := -1
		for i := 0; i < len(students); i++ {
			if students[(i+sIdx)%len(students)] == sandwiches[swIdx] {
				isFound = true
				idx = (i + sIdx) % len(students)
				break
			}
		}
		if !isFound {
			break
		}
		students[idx] = -1
		sIdx = (idx + 1) % len(students)
		swIdx++
	}
	count := 0
	for i := 0; i < len(students); i++ {
		if students[i] != -1 {
			count++
		}
	}
	return count
}

func main() {
	fmt.Println()
}
