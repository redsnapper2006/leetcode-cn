package main

import "fmt"

func minSideJumps(obstacles []int) int {
	steps := 0
	idx := 2
	isSingle := true
	index := []int{1, 2, 3}
	for i := 0; i < len(obstacles)-1; i++ {
		if isSingle {
			if obstacles[i+1] != idx {
				continue
			}
			steps++
			match := -1
			for j := 0; j < len(index); j++ {
				if index[j] == idx {
					continue
				}
				if obstacles[i] == index[j] {
					match = index[j]
				}
			}
			if match == -1 {
				isSingle = false
			} else {
				for j := 0; j < len(index); j++ {
					if index[j] == idx || index[j] == match {
						continue
					}
					idx = index[j]
					break
				}
				isSingle = true
			}
		} else {
			match := -1
			for j := 0; j < len(index); j++ {
				if index[j] == idx {
					continue
				}
				if obstacles[i+1] == index[j] {
					match = index[j]
				}
			}
			if match != -1 {
				for j := 0; j < len(index); j++ {
					if index[j] == idx || index[j] == match {
						continue
					}
					idx = index[j]
					break
				}
				isSingle = true
			}
		}

	}
	return steps
}

func main() {
	fmt.Println("")
}
