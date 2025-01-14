package main

import "fmt"

func merge(intervals [][]int) [][]int {
	min, max := 100000, -1
	for _, v := range intervals {
		if min > v[0] {
			min = v[0]
		}
		if max < v[1] {
			max = v[1]
		}
	}
	startMap := map[int]int{}
	buf := make([]int, max+1)
	for _, v := range intervals {
		buf[v[0]]++
		buf[v[1]]--
		startMap[v[0]] = 1
	}

	ret := [][]int{}
	sum := 0
	start := -1
	isIn := false
	for i := min; i < max+1; i++ {
		sum += buf[i]
		if sum == 0 && isIn {
			ret = append(ret, []int{start, i})
			isIn = false
		} else if !isIn && sum > 0 {
			isIn = true
			start = i
		} else {
			_, ok := startMap[i]
			if ok && sum == 0 {
				ret = append(ret, []int{i, i})
				isIn = false
			}
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
