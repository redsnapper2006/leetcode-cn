package main

import "fmt"

func maxDistance(colors []int) int {
	m := map[int][]int{}
	for i, c := range colors {
		v, ok := m[c]
		if !ok {
			m[c] = []int{i, i}
		} else {
			m[c] = []int{v[0], i}
		}
	}
	diff := [][]int{}
	for _, v := range m {
		diff = append(diff, v)
	}
	max := 0
	for i := 0; i < len(diff); i++ {
		for j := i + 1; j < len(diff); j++ {
			d := diff[i][0] - diff[j][1]
			if d < 0 {
				d = -d
			}
			if d > max {
				max = d
			}
			d = diff[i][1] - diff[j][0]
			if d < 0 {
				d = -d
			}
			if d > max {
				max = d
			}
		}
	}
	return max
}

func main() {
	fmt.Println()
}
