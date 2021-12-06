package main

import "fmt"

func minimumOperations(nums []int, start int, goal int) int {
	if start == goal {
		return 0
	}

	buf := make([]int, 1001)
	candi := []int{start}
	steps := 0

	for {
		steps++
		t := []int{}
		for _, c := range candi {
			for _, n := range nums {
				next := c + n
				if next == goal {
					return steps
				}
				if next >= 0 && next <= 1000 && buf[next] == 0 {
					t = append(t, next)
					buf[next] = steps
				}
				next = c - n
				if next == goal {
					return steps
				}
				if next >= 0 && next <= 1000 && buf[next] == 0 {
					t = append(t, next)
					buf[next] = steps
				}
				next = c ^ n
				if next == goal {
					return steps
				}
				if next >= 0 && next <= 1000 && buf[next] == 0 {
					t = append(t, next)
					buf[next] = steps
				}
			}
		}
		if len(t) == 0 {
			return -1
		}
		candi = t
	}
}

func main() {
	fmt.Println()
}
